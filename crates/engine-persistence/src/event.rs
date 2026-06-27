use shared_models::{NormalizedTelemetryEvent, TelemetryEventId, Timestamp};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersistenceKind {
    RegistryRunKey,
    StartupFolder,
    ScheduledTask,
    Service,
    WmiSubscription,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistenceEvent {
    pub kind: PersistenceKind,
    pub path: String,
    pub value: String,
    pub operation: String,
    pub image_path: Option<String>,
    pub command: Option<String>,
    pub user: Option<String>,
    pub supporting_event_id: TelemetryEventId,
    pub observed_at: Timestamp,
}

impl PersistenceEvent {
    pub fn from_telemetry(event: &NormalizedTelemetryEvent) -> Option<Self> {
        let path = event.metadata.get("persistence.path")?.to_string();
        let value = event
            .metadata
            .get("persistence.value")
            .unwrap_or_default()
            .to_string();
        let operation = event
            .metadata
            .get("persistence.operation")
            .unwrap_or_default()
            .to_string();
        let kind_hint = event.metadata.get("persistence.kind").unwrap_or_default();
        let kind = classify_kind(kind_hint, &path, &value);

        Some(Self {
            kind,
            path,
            value,
            operation,
            image_path: event.metadata.get("persistence.image_path").map(str::to_string),
            command: event.metadata.get("persistence.command").map(str::to_string),
            user: event.metadata.get("persistence.user").map(str::to_string),
            supporting_event_id: event.event_id.clone(),
            observed_at: event.timestamp.clone(),
        })
    }
}

fn classify_kind(kind_hint: &str, path: &str, value: &str) -> PersistenceKind {
    let kind_hint = normalize(kind_hint);
    let path = normalize(path);
    let value = normalize(value);
    let combined = format!("{kind_hint} {path} {value}");

    if combined.contains("runonce")
        || combined.contains("registry_run_key")
        || combined.contains(r"\software\microsoft\windows\currentversion\run")
    {
        PersistenceKind::RegistryRunKey
    } else if combined.contains("startup_folder")
        || combined.contains(r"\start menu\programs\startup\")
        || combined.contains(r"\startup\")
    {
        PersistenceKind::StartupFolder
    } else if combined.contains("scheduled_task")
        || combined.contains(r"\system32\tasks\")
        || combined.contains(r"\microsoft\windows\task scheduler\")
    {
        PersistenceKind::ScheduledTask
    } else if combined.contains("service")
        || combined.contains(r"\system\currentcontrolset\services\")
    {
        PersistenceKind::Service
    } else if combined.contains("wmi")
        || combined.contains("__eventfilter")
        || combined.contains("commandlineeventconsumer")
        || combined.contains("__filtertoconsumerbinding")
    {
        PersistenceKind::WmiSubscription
    } else {
        PersistenceKind::Unknown
    }
}

fn normalize(value: &str) -> String {
    value.replace('/', r"\").to_ascii_lowercase()
}
