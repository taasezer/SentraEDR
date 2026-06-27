use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperatingMode {
    AuditOnly,
    Interactive,
    Automatic,
}

#[derive(Debug, Clone)]
pub struct SafetyPolicy {
    pub policy_id: String,
    pub policy_version: String,
    pub operating_mode: OperatingMode,
    pub requires_reversibility: bool,
}

#[derive(Debug, Clone)]
pub struct RemediationAction {
    pub action_id: Uuid,
    pub alert_id: Uuid,
    pub provider_id: String, // e.g., "ProcessTerminator"
    pub payload: String, // e.g., PID or Path
}

#[derive(Debug, Clone)]
pub struct RollbackData {
    pub action_id: Uuid,
    pub snapshot_state: String, // E.g., serialized registry hive backup
}

#[derive(Debug, Clone)]
pub struct AuditRecord {
    pub alert_id: Uuid,
    pub action_id: Uuid,
    pub timestamp_ms: u64,
    pub policy_id: String,
    pub result_status: String,
    pub rollback_reference: Option<Uuid>,
    pub integrity_hash: String, // Cryptographic hash placeholder
}

// Typestate models for the state machine
pub struct StateAlertReceived { pub alert_id: Uuid }
pub struct StateActionPlanned { pub action: RemediationAction }
pub struct StatePendingApproval { pub action: RemediationAction }
pub struct StateSafetyValidated { pub action: RemediationAction, pub rollback: Option<RollbackData> }
pub struct StateExecuting { pub action: RemediationAction, pub rollback: Option<RollbackData> }
pub struct StateVerification { pub action: RemediationAction, pub rollback: Option<RollbackData> }
pub struct StateCompleted { pub audit: AuditRecord }
