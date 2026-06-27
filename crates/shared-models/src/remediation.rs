use crate::detection::AlertId;
use crate::time::Timestamp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemediationMode {
    ObserveOnly,
    ApprovalRequired,
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemediationAction {
    SuspendProcess,
    IsolateNetwork,
    QuarantineFile,
    BackupRegistryValue,
    RestoreRegistryValue,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemediationCommand {
    pub command_id: Uuid,
    pub alert_id: AlertId,
    pub requested_at: Timestamp,
    pub requested_by: String,
    pub mode: RemediationMode,
    pub action: RemediationAction,
    pub rationale: String,
}

impl RemediationCommand {
    pub fn new(
        alert_id: AlertId,
        requested_at: Timestamp,
        requested_by: impl Into<String>,
        mode: RemediationMode,
        action: RemediationAction,
        rationale: impl Into<String>,
    ) -> Self {
        Self {
            command_id: Uuid::new_v4(),
            alert_id,
            requested_at,
            requested_by: requested_by.into(),
            mode,
            action,
            rationale: rationale.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemediationStatus {
    RejectedByPolicy,
    WaitingForApproval,
    Approved,
    Completed,
    Failed,
}
