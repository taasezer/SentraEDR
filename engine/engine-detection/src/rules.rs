use crate::models::Alert;
use crate::pipeline::CorrelationState;

/// Defines the deterministic logic for a behavioral rule.
pub trait Rule {
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
