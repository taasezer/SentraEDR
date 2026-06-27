use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct Evidence {
    /// References to the exact immutable events that triggered the alert.
    pub related_event_ids: Vec<Uuid>,
    /// A human-readable reasoning path for explainability.
    pub reasoning_path: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Alert {
    pub alert_id: Uuid,
    pub rule_id: String,
    pub severity: u8,   // 1-100 Risk Score
    pub confidence: u8, // 1-100 Confidence Score
    pub timestamp_ms: u64,
    pub related_process_id: Option<u32>, // Tied to ProcessIdentity conceptually
    pub evidence: Evidence,
}
