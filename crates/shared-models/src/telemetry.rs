use crate::process::ProcessIdentity;
use crate::time::Timestamp;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaVersion {
    pub major: u16,
    pub minor: u16,
}

impl SchemaVersion {
    pub const V1: Self = Self { major: 1, minor: 0 };
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelemetryEventId(Uuid);

impl TelemetryEventId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TelemetryEventId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EventPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TelemetrySource {
    Etw,
    Sysmon,
    WindowsEventLog,
    InternalHealth,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TelemetryAction {
    ProcessStarted,
    ProcessExited,
    ImageLoaded,
    RegistryChanged,
    PowerShellExecuted,
    NetworkConnectionObserved,
    MemoryEventObserved,
    ComponentHealthChanged,
    FileCreated,
    FileModified,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelemetryMetadata {
    values: BTreeMap<String, String>,
}

impl TelemetryMetadata {
    pub fn empty() -> Self {
        Self {
            values: BTreeMap::new(),
        }
    }

    pub fn insert(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.values.insert(key.into(), value.into());
        self
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }
}

impl Default for TelemetryMetadata {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedTelemetryEvent {
    pub schema_version: SchemaVersion,
    pub event_id: TelemetryEventId,
    pub timestamp: Timestamp,
    pub source: TelemetrySource,
    pub priority: EventPriority,
    pub process: Option<ProcessIdentity>,
    pub action: TelemetryAction,
    pub metadata: TelemetryMetadata,
    pub confidence_hint: u8,
}

impl NormalizedTelemetryEvent {
    pub fn new(
        source: TelemetrySource,
        priority: EventPriority,
        action: TelemetryAction,
        timestamp: Timestamp,
    ) -> Self {
        Self {
            schema_version: SchemaVersion::V1,
            event_id: TelemetryEventId::new(),
            timestamp,
            source,
            priority,
            process: None,
            action,
            metadata: TelemetryMetadata::empty(),
            confidence_hint: 0,
        }
    }

    pub fn with_process(mut self, process: ProcessIdentity) -> Self {
        self.process = Some(process);
        self
    }

    pub fn with_confidence_hint(mut self, confidence_hint: u8) -> Self {
        self.confidence_hint = confidence_hint.min(100);
        self
    }
}
