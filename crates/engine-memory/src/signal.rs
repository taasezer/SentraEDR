use crate::event::{MemoryEvent, MemoryEventKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalSeverity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemorySignal {
    pub name: String,
    pub description: String,
    pub severity: SignalSeverity,
    pub event: MemoryEvent,
}

pub fn signal_for_event(event: &MemoryEvent) -> Option<MemorySignal> {
    match event.kind {
        MemoryEventKind::RemoteThreadCreated => Some(signal(
            "remote_thread_creation",
            "Memory telemetry indicates remote thread creation metadata",
            SignalSeverity::High,
            event,
        )),
        MemoryEventKind::ExecutablePrivateMemory => Some(signal(
            "executable_private_memory",
            "Memory telemetry indicates executable private memory metadata",
            SignalSeverity::High,
            event,
        )),
        MemoryEventKind::UnsignedModuleLoaded => Some(signal(
            "unsigned_module_loaded",
            "Memory telemetry indicates unsigned module load metadata",
            SignalSeverity::Medium,
            event,
        )),
        MemoryEventKind::SectionMapping => Some(signal(
            "suspicious_section_mapping",
            "Memory telemetry indicates suspicious section mapping metadata",
            SignalSeverity::Medium,
            event,
        )),
        MemoryEventKind::ProtectionChanged => {
            let protection = event
                .protection
                .as_deref()
                .unwrap_or_default()
                .to_ascii_lowercase();
            if protection.contains("execute") {
                Some(signal(
                    "memory_protection_escalation",
                    "Memory telemetry indicates protection changed to executable permissions",
                    SignalSeverity::High,
                    event,
                ))
            } else {
                None
            }
        }
        MemoryEventKind::Unknown => None,
    }
}

fn signal(
    name: &str,
    description: &str,
    severity: SignalSeverity,
    event: &MemoryEvent,
) -> MemorySignal {
    MemorySignal {
        name: name.to_string(),
        description: description.to_string(),
        severity,
        event: event.clone(),
    }
}
