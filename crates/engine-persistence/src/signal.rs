use crate::event::{PersistenceEvent, PersistenceKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalSeverity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistenceSignal {
    pub name: String,
    pub description: String,
    pub severity: SignalSeverity,
    pub event: PersistenceEvent,
}

pub fn signal_for_event(event: &PersistenceEvent) -> Option<PersistenceSignal> {
    match event.kind {
        PersistenceKind::RegistryRunKey => Some(PersistenceSignal {
            name: "registry_run_key_persistence".to_string(),
            description: "Persistence metadata indicates Run or RunOnce key modification"
                .to_string(),
            severity: SignalSeverity::High,
            event: event.clone(),
        }),
        PersistenceKind::StartupFolder => Some(PersistenceSignal {
            name: "startup_folder_persistence".to_string(),
            description: "Persistence metadata indicates startup folder modification".to_string(),
            severity: SignalSeverity::Medium,
            event: event.clone(),
        }),
        PersistenceKind::ScheduledTask => Some(PersistenceSignal {
            name: "scheduled_task_persistence".to_string(),
            description: "Persistence metadata indicates scheduled task creation or modification"
                .to_string(),
            severity: SignalSeverity::Medium,
            event: event.clone(),
        }),
        PersistenceKind::Service => Some(PersistenceSignal {
            name: "service_persistence".to_string(),
            description: "Persistence metadata indicates service creation or service path change"
                .to_string(),
            severity: SignalSeverity::High,
            event: event.clone(),
        }),
        PersistenceKind::WmiSubscription => Some(PersistenceSignal {
            name: "wmi_persistence".to_string(),
            description: "Persistence metadata indicates WMI permanent event subscription"
                .to_string(),
            severity: SignalSeverity::High,
            event: event.clone(),
        }),
        PersistenceKind::Unknown => None,
    }
}
