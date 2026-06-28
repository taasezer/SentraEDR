use std::collections::HashMap;

/// A platform-agnostic, normalized telemetry event.
/// This structure must contain NO Windows-specific types (e.g., GUID, HRESULT).
#[derive(Debug, Clone)]
pub struct NormalizedTelemetryEvent {
    /// A globally unique identifier for this specific event occurrence.
    pub event_id: uuid::Uuid,
    /// The schema version of this event to allow backwards-compatible evolution.
    pub schema_version: u16,
    /// Timestamp of when the event occurred (in Unix Epoch or similar generic time format).
    pub timestamp_ms: u64,
    /// The unique identifier of the process associated with this event (e.g., PID).
    pub process_id: u32,
    /// The parent process ID, if applicable.
    pub parent_process_id: Option<u32>,
    /// The specific type of the event.
    pub event_type: EventType,
    /// Generic key-value metadata for extensible schema mapping without modifying the core struct.
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventType {
    ProcessCreate {
        command_line: String,
        image_path: String,
    },
    ProcessExit {
        exit_code: u32,
    },
    RemoteThreadCreate {
        target_process_id: u32,
    },
    ImageLoad {
        image_path: String,
        is_signed: bool,
    },
    NetworkConnection {
        destination_ip: String,
        destination_port: u16,
        protocol: String,
    },
    RegistryActivity {
        key_path: String,
        value_name: String,
        action: String,
    },
    PowerShellExecution {
        script_block: String,
    },
    FileActivity {
        file_path: String,
        action: String,
    },
    Unknown,
}
