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
    let desc_slice = std::slice::from_ref(&desc);
    
    let status = TdhGetPropertySize(record, None, desc_slice, &mut buffer_size);
    if status != 0 || buffer_size == 0 { return None; }

    let mut buffer = vec![0u8; buffer_size as usize];
    let status = TdhGetProperty(record, None, desc_slice, &mut buffer);
    if status != 0 { return None; }

    let u16_slice = std::slice::from_raw_parts(buffer.as_ptr() as *const u16, (buffer_size / 2) as usize);
    let len = u16_slice.iter().position(|&c| c == 0).unwrap_or(u16_slice.len());
    Some(String::from_utf16_lossy(&u16_slice[..len]))
}

unsafe fn extract_u32_property(record: *mut EVENT_RECORD, property_name: &str) -> Option<u32> {
    let mut utf16_name = property_name.encode_utf16().chain(std::iter::once(0)).collect::<Vec<u16>>();
    let mut desc = PROPERTY_DATA_DESCRIPTOR::default();
    desc.PropertyName = utf16_name.as_mut_ptr() as u64;
    desc.ArrayIndex = 0xFFFFFFFF;
    
    let mut value: u32 = 0;
    let desc_slice = std::slice::from_ref(&desc);
    
    let mut buffer_size: u32 = std::mem::size_of::<u32>() as u32;
    let status = TdhGetProperty(record, None, desc_slice, std::slice::from_raw_parts_mut(&mut value as *mut u32 as *mut u8, buffer_size as usize));
    
    if status == 0 { Some(value) } else { None }
}

unsafe fn extract_u16_property(record: *mut EVENT_RECORD, property_name: &str) -> Option<u16> {
    let mut utf16_name = property_name.encode_utf16().chain(std::iter::once(0)).collect::<Vec<u16>>();
    let mut desc = PROPERTY_DATA_DESCRIPTOR::default();
    desc.PropertyName = utf16_name.as_mut_ptr() as u64;
    desc.ArrayIndex = 0xFFFFFFFF;
    
    let mut value: u16 = 0;
    let desc_slice = std::slice::from_ref(&desc);
    
    let mut buffer_size: u32 = std::mem::size_of::<u16>() as u32;
    let status = TdhGetProperty(record, None, desc_slice, std::slice::from_raw_parts_mut(&mut value as *mut u16 as *mut u8, buffer_size as usize));
    
    if status == 0 { Some(value) } else { None }
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
    let mut process_id = record_ref.EventHeader.ProcessId;
    let event_id = record_ref.EventHeader.EventDescriptor.Id;

    let mut event_type = shared_models::events::EventType::Unknown;

    // KERNEL_PROCESS_GUID
    if provider_id.data1 == 0x22fb2cd6 {
        if event_id == 1 { // Process Start
            let image_name = unsafe { extract_string_property(record, "ImageName") }
                .unwrap_or_else(|| "Unknown".to_string());
            let command_line = unsafe { extract_string_property(record, "CommandLine") }
                .unwrap_or_else(|| "Unknown".to_string());

            event_type = shared_models::events::EventType::ProcessCreate {
                image_path: image_name,
                command_line,
            };
        } else if event_id == 3 { // Thread Start (Used for Process Injection)
            // The process ID is the Target. The Creator process is the Attacker.
            let creator_pid = unsafe { extract_u32_property(record, "CreatorProcessId") }.unwrap_or(process_id);
            if creator_pid != process_id && creator_pid != 0 && process_id != 0 {
                event_type = shared_models::events::EventType::RemoteThreadCreate {
                    target_process_id: process_id,
                };
                // We overwrite process_id here to reflect the ATTACKER so the Auto-Kill targets the attacker
                process_id = creator_pid;
            }
        }
    } 
    // KERNEL_NETWORK_GUID
    else if provider_id.data1 == 0x7dd42a49 {
        if event_id == 10 || event_id == 11 { // TCP Send/Receive
            let daddr = unsafe { extract_u32_property(record, "daddr") }.unwrap_or(0);
            let dport = unsafe { extract_u16_property(record, "dport") }.unwrap_or(0);
            
            // Convert u32 to IP string (simple format for demo)
            let ip = format!("{}.{}.{}.{}", (daddr & 0xFF), (daddr >> 8) & 0xFF, (daddr >> 16) & 0xFF, (daddr >> 24) & 0xFF);
            
            event_type = shared_models::events::EventType::NetworkConnection {
                destination_ip: ip,
                destination_port: dport.to_be(), // Ports are usually big-endian
                protocol: "TCP".to_string(),
            };
        } else if event_id == 14 || event_id == 15 { // UDP Send/Receive
            let daddr = unsafe { extract_u32_property(record, "daddr") }.unwrap_or(0);
            let dport = unsafe { extract_u16_property(record, "dport") }.unwrap_or(0);
            let ip = format!("{}.{}.{}.{}", (daddr & 0xFF), (daddr >> 8) & 0xFF, (daddr >> 16) & 0xFF, (daddr >> 24) & 0xFF);
            
            event_type = shared_models::events::EventType::NetworkConnection {
                destination_ip: ip,
                destination_port: dport.to_be(),
                protocol: "UDP".to_string(),
            };
        }
    }
    // KERNEL_FILE_GUID
    else if provider_id.data1 == 0xedd08927 {
        if event_id == 64 || event_id == 14 || event_id == 15 { // Create/Write/Rename
            let file_name = unsafe { extract_string_property(record, "FileName") }
                .unwrap_or_else(|| "Unknown".to_string());
            
            let action = match event_id {
                64 => "Create",
                14 => "Write",
                15 => "Rename",
                _ => "Unknown"
            };

            event_type = shared_models::events::EventType::FileActivity {
                file_path: file_name,
                action: action.to_string(),
            };
        }
    }

    // KERNEL_REGISTRY_GUID
    else if provider_id.data1 == 0x70eb4f03 {
        // Event ID 1 = CreateKey, 5 = SetValueKey
        if event_id == 5 {
            let key_name = unsafe { extract_string_property(record, "KeyName") }
                .unwrap_or_else(|| "Unknown".to_string());
            let value_name = unsafe { extract_string_property(record, "ValueName") }
                .unwrap_or_else(|| "Unknown".to_string());
            
            event_type = shared_models::events::EventType::RegistryActivity {
                key_path: key_name,
                value_name,
                action: "SetValue".to_string(),
            };
        }
    }

    // Drop unknown events to save memory and CPU
    if matches!(event_type, shared_models::events::EventType::Unknown) {
        return;
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
