use shared_models::{
    NormalizedTelemetryEvent, TelemetryAction, TelemetryEventId, Timestamp,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryEventKind {
    RemoteThreadCreated,
    ExecutablePrivateMemory,
    UnsignedModuleLoaded,
    SectionMapping,
    ProtectionChanged,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryEvent {
    pub kind: MemoryEventKind,
    pub source_process_id: Option<u32>,
    pub target_process_id: Option<u32>,
    pub module_path: Option<String>,
    pub protection: Option<String>,
    pub region_kind: Option<String>,
    pub allocation_size: Option<u64>,
    pub thread_start_address: Option<String>,
    pub supporting_event_id: TelemetryEventId,
    pub observed_at: Timestamp,
}

impl MemoryEvent {
    pub fn from_telemetry(event: &NormalizedTelemetryEvent) -> Option<Self> {
        if event.action != TelemetryAction::MemoryEventObserved {
            return None;
        }

        let event_type = event.metadata.get("memory.event_type")?;
        let kind = classify_kind(event_type);

        Some(Self {
            kind,
            source_process_id: event
                .metadata
                .get("memory.source_process_id")
                .and_then(|value| value.parse().ok()),
            target_process_id: event
                .metadata
                .get("memory.target_process_id")
                .and_then(|value| value.parse().ok()),
            module_path: event.metadata.get("memory.module_path").map(str::to_string),
            protection: event.metadata.get("memory.protection").map(str::to_string),
            region_kind: event.metadata.get("memory.region_kind").map(str::to_string),
            allocation_size: event
                .metadata
                .get("memory.allocation_size")
                .and_then(|value| value.parse().ok()),
            thread_start_address: event
                .metadata
                .get("memory.thread_start_address")
                .map(str::to_string),
            supporting_event_id: event.event_id.clone(),
            observed_at: event.timestamp.clone(),
        })
    }
}

fn classify_kind(value: &str) -> MemoryEventKind {
    match value.to_ascii_lowercase().as_str() {
        "remote_thread_created" | "remote_thread" => MemoryEventKind::RemoteThreadCreated,
        "executable_private_memory" | "private_execute" => {
            MemoryEventKind::ExecutablePrivateMemory
        }
        "unsigned_module_loaded" | "unsigned_module" => MemoryEventKind::UnsignedModuleLoaded,
        "section_mapping" | "mapped_section" => MemoryEventKind::SectionMapping,
        "protection_changed" | "memory_protection_changed" => {
            MemoryEventKind::ProtectionChanged
        }
        _ => MemoryEventKind::Unknown,
    }
}
