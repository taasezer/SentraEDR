use crate::severity::Severity;
use uuid::Uuid;

pub struct EventMetadata {
    pub event_id: u32,
    pub severity: Severity,
    pub component_id: String,
    pub correlation_id: Uuid,
    pub timestamp_ns: u64,
}
