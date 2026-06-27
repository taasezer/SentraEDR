use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PersistedEvent {
    pub internal_id: Uuid,
    pub schema_version: u32,
    pub event_version: u32,
    pub producer_version: String,
    pub timestamp_ms: u64,
    pub payload_json: String, // Or bincode
}

#[derive(Debug, Clone)]
pub struct RetentionPolicy {
    pub max_age_days: u32,
    pub max_size_mb: u32,
}
