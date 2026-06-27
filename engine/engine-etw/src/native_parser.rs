use crossbeam_channel::Sender;
use shared_models::events::NormalizedTelemetryEvent;
use std::sync::OnceLock;
use windows::Win32::System::Diagnostics::Etw::{
    EVENT_HEADER, EVENT_RECORD, PROPERTY_DATA_DESCRIPTOR, TdhGetPropertySize, TdhGetProperty,
};
use windows::core::PWSTR;
pub static EVENT_SENDER: OnceLock<Sender<NormalizedTelemetryEvent>> = OnceLock::new();

unsafe fn extract_string_property(record: *mut EVENT_RECORD, property_name: &str) -> Option<String> {
    let mut utf16_name = property_name.encode_utf16().chain(std::iter::once(0)).collect::<Vec<u16>>();
    
    let mut desc = PROPERTY_DATA_DESCRIPTOR::default();
    desc.PropertyName = utf16_name.as_mut_ptr() as u64;
    desc.ArrayIndex = 0xFFFFFFFF; // ULONG_MAX for single-valued properties

    let mut buffer_size: u32 = 0;
    
    // In windows-rs 0.58.0, TdhGetPropertySize expects &[PROPERTY_DATA_DESCRIPTOR]
    let desc_slice = std::slice::from_ref(&desc);
    
    let status = TdhGetPropertySize(
        record,
        None,
        desc_slice,
        &mut buffer_size,
    );

    // WIN32_ERROR(0) is ERROR_SUCCESS
    if status != 0 || buffer_size == 0 {
        return None;
    }

    let mut buffer = vec![0u8; buffer_size as usize];
    
    let status = TdhGetProperty(
        record,
        None,
        desc_slice,
        &mut buffer,
    );

    if status != 0 {
        return None;
    }

    // Convert raw UTF-16 bytes (excluding null terminator) to String
    let u16_slice = std::slice::from_raw_parts(
        buffer.as_ptr() as *const u16,
        (buffer_size / 2) as usize,
    );
    
    // Find null terminator if it exists
    let len = u16_slice.iter().position(|&c| c == 0).unwrap_or(u16_slice.len());
    Some(String::from_utf16_lossy(&u16_slice[..len]))
}

/// Raw C-style callback for `ProcessTrace`.
/// This function blocks the ETW OS Thread and parses every event synchronously,
/// dispatching it to the crossbeam channel for Tokio to consume.
pub extern "system" fn event_record_callback(record: *mut EVENT_RECORD) {
    if record.is_null() {
        return;
    }

    // Safety: record pointer is guaranteed valid for the lifetime of this callback by ETW.
    let record_ref = unsafe { &*record };

    // Very simple extraction for Phase 17 Process Events proof of concept.
    // GUID for Microsoft-Windows-Kernel-Process
    let provider_id = record_ref.EventHeader.ProviderId;
    let process_id = record_ref.EventHeader.ProcessId;
    let event_id = record_ref.EventHeader.EventDescriptor.Id;

    let mut event_type = shared_models::events::EventType::Unknown;

    if event_id == 1 { // Process Start
        let image_name = unsafe { extract_string_property(record, "ImageName") }
            .unwrap_or_else(|| "Unknown".to_string());
        
        let command_line = unsafe { extract_string_property(record, "CommandLine") }
            .unwrap_or_else(|| "Unknown".to_string());

        event_type = shared_models::events::EventType::ProcessCreate {
            image_path: image_name,
            command_line,
        };
    }

    // Dispatch the raw event immediately into the bounded channel.
    if let Some(sender) = EVENT_SENDER.get() {
        let event = NormalizedTelemetryEvent {
            event_id: uuid::Uuid::new_v4(),
            schema_version: 1,
            timestamp_ms: chrono::Utc::now().timestamp_millis() as u64,
            process_id,
            parent_process_id: Some(0), // Can be extracted if needed
            event_type,
            metadata: std::collections::HashMap::new(),
        };

        // We use try_send to explicitly drop events if the Tokio consumer is lagging.
        // This enforces our Memory Bounding rule.
        let _ = sender.try_send(event);
    }
}
