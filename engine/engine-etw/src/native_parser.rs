use crossbeam_channel::Sender;
use shared_models::telemetry::NormalizedTelemetryEvent;
use std::sync::OnceLock;
use windows::Win32::System::Diagnostics::Etw::{EVENT_HEADER, EVENT_RECORD};

// Global sender for the C-callback. Initialized once per trace session.
pub static EVENT_SENDER: OnceLock<Sender<NormalizedTelemetryEvent>> = OnceLock::new();

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

    // Dispatch the raw event immediately into the bounded channel.
    if let Some(sender) = EVENT_SENDER.get() {
        let event = NormalizedTelemetryEvent {
            timestamp: chrono::Utc::now(),
            sensor_id: "etw-kernel".to_string(),
            event_type: format!("Provider: {:?} EventId: {}", provider_id, event_id),
            process_id,
            parent_process_id: 0,
            image_path: String::new(), // In a real parser we use TdhGetProperty
            command_line: None,
            user_sid: None,
            severity: 1,
            payload: serde_json::json!({"raw_event_id": event_id}),
        };

        // We use try_send to explicitly drop events if the Tokio consumer is lagging.
        // This enforces our Memory Bounding rule.
        let _ = sender.try_send(event);
    }
}
