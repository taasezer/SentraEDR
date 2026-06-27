use crate::models::Alert;
use crate::pipeline::CorrelationState;
use shared_models::events::EventType;

/// Defines the deterministic logic for a behavioral rule.
pub trait Rule: Send + Sync {
    /// Unique identifier (e.g., "EDR-NET-001")
    fn rule_id(&self) -> &str;

    /// The maximum historical context this rule needs to evaluate successfully.
    /// The Correlation Pipeline will use the maximum of all active rules to enforce bounds.
    fn max_correlation_window_ms(&self) -> u64;

    /// Evaluates the current state and returns an Alert if the logic triggers.
    /// The Risk and Confidence scores are generated here based on the data.
    fn evaluate(&self, state: &CorrelationState) -> Option<Alert>;
}

/// A simulated mock rule: "Suspicious Process Network Connection"
pub struct SuspiciousNetworkRule;

impl Rule for SuspiciousNetworkRule {
    fn rule_id(&self) -> &str {
        "EDR-NET-001"
    }

    fn max_correlation_window_ms(&self) -> u64 {
        // This rule only needs to look back 5 seconds (5000ms) to correlate a ProcessCreate to a NetworkConnection.
        5000
    }

    fn evaluate(&self, _state: &CorrelationState) -> Option<Alert> {
        // In a real implementation, this scans the state index for the sequence.
        None
    }
}

pub struct SuspiciousProcessRule;

impl Rule for SuspiciousProcessRule {
    fn rule_id(&self) -> &str {
        "EDR-PROC-001"
    }

    fn max_correlation_window_ms(&self) -> u64 {
        0 // Only requires the most recent event
    }

    fn evaluate(&self, state: &CorrelationState) -> Option<Alert> {
        if let Some(event) = state.events.back() {
            if let EventType::ProcessCreate { image_path, command_line } = &event.event_type {
                let image_lower = image_path.to_lowercase();
                if image_lower.contains("powershell.exe") || image_lower.contains("pwsh.exe") || image_lower.contains("cmd.exe") {
                    return Some(Alert {
                        alert_id: uuid::Uuid::new_v4(),
                        rule_id: self.rule_id().to_string(),
                        timestamp_ms: event.timestamp_ms,
                        severity: 90, // High Risk
                        confidence: 90, // High Confidence
                        related_process_id: Some(event.process_id),
                        evidence: crate::models::Evidence {
                            related_event_ids: vec![event.event_id],
                            reasoning_path: format!("A suspicious process was spawned: {} with args: {}", image_path, command_line),
                        }
                    });
                }
            }
        }
        None
    }
}
