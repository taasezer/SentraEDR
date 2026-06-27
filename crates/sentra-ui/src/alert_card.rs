use shared_models::{Alert, AlertId, RiskLevel, Timestamp};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlertCard {
    pub alert_id: AlertId,
    pub timestamp: Timestamp,
    pub risk_level: RiskLevel,
    pub score: u8,
    pub signal_count: usize,
    pub mitre_technique_count: usize,
    pub recommended_action: String,
    pub remediation_eligible: bool,
}

impl AlertCard {
    pub fn from_alert(alert: Alert) -> Self {
        Self {
            alert_id: alert.alert_id,
            timestamp: alert.finding.timestamp,
            risk_level: alert.finding.risk_level,
            score: alert.finding.score,
            signal_count: alert.finding.signals.len(),
            mitre_technique_count: alert.finding.mitre_techniques.len(),
            recommended_action: alert.recommended_action,
            remediation_eligible: alert.remediation_eligible,
        }
    }
}
