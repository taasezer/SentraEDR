use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventPriority {
    Low,
    Normal,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventCategory {
    Telemetry,
    Detection,
    Remediation,
    Infrastructure,
    Audit,
    Health,
}

#[derive(Debug, Clone)]
pub struct MessageMetadata {
    pub message_id: Uuid,
    pub correlation_id: Option<Uuid>,
    pub causation_id: Option<Uuid>,
    pub timestamp_ms: u64,
    pub producer_id: String,
    pub schema_version: u32,
}

pub trait EventMessage: Send + Sync + Clone + std::fmt::Debug {
    fn metadata(&self) -> &MessageMetadata;
    fn category(&self) -> EventCategory;
    fn priority(&self) -> EventPriority;
}

pub trait CommandMessage: Send + Sync + std::fmt::Debug {
    fn metadata(&self) -> &MessageMetadata;
}
