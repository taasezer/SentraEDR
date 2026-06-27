use uuid::Uuid;

/// Globally unique process identity preventing PID collision errors.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ProcessIdentity {
    pub pid: u32,
    pub creation_time_ms: u64,
    pub original_event_id: Uuid,
}

/// Static, immutable metadata about a process collected once.
#[derive(Debug, Clone)]
pub struct ProcessMetadata {
    pub image_path: String,
    pub command_line: String,
    pub session_id: u32,
    pub initial_user_sid: String,
}

/// A dynamic state transition during the process lifetime.
#[derive(Debug, Clone)]
pub enum ProcessStateChange {
    TokenElevated { new_integrity: String },
    ThreadInjected { source_pid: u32 },
}

/// A point-in-time snapshot merging static metadata and current dynamic state.
/// Emitted for the Detection Engine to consume.
#[derive(Debug, Clone)]
pub struct ProcessSnapshot {
    pub identity: ProcessIdentity,
    pub parent_identity: Option<ProcessIdentity>,
    pub metadata: ProcessMetadata,
    pub current_integrity_level: String,
}
