use crate::process::ProcessIdentity;
use crate::telemetry::TelemetryEventId;
use crate::time::Timestamp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FindingId(Uuid);

impl FindingId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for FindingId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlertId(Uuid);

impl AlertId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for AlertId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Informational,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signal {
    pub name: String,
    pub description: String,
    pub supporting_event_ids: Vec<TelemetryEventId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finding {
    pub finding_id: FindingId,
    pub timestamp: Timestamp,
    pub risk_level: RiskLevel,
    pub score: u8,
    pub process: Option<ProcessIdentity>,
    pub signals: Vec<Signal>,
    pub mitre_techniques: Vec<String>,
    pub telemetry_uncertainty: bool,
}

impl Finding {
    pub fn new(timestamp: Timestamp, risk_level: RiskLevel, score: u8) -> Self {
        Self {
            finding_id: FindingId::new(),
            timestamp,
            risk_level,
            score: score.min(100),
            process: None,
            signals: Vec::new(),
            mitre_techniques: Vec::new(),
            telemetry_uncertainty: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Alert {
    pub alert_id: AlertId,
    pub finding: Finding,
    pub recommended_action: String,
    pub remediation_eligible: bool,
}

impl Alert {
    pub fn observe_only(finding: Finding, recommended_action: impl Into<String>) -> Self {
        Self {
            alert_id: AlertId::new(),
            finding,
            recommended_action: recommended_action.into(),
            remediation_eligible: false,
        }
    }
}
