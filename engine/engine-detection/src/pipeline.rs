use shared_models::events::NormalizedTelemetryEvent;
use crate::models::Alert;
use crate::rules::Rule;
use crate::metrics::METRICS;
use std::collections::VecDeque;

/// Bounded, searchable memory state for event correlation.
pub struct CorrelationState {
    /// A simple chronological buffer for the mock implementation.
    /// In production, this uses multiple HashMaps indexed by ProcessIdentity and EventType.
    pub events: VecDeque<NormalizedTelemetryEvent>,
    max_capacity: usize,
}

impl CorrelationState {
    pub fn new(max_capacity: usize) -> Self {
        Self {
            events: VecDeque::with_capacity(max_capacity),
            max_capacity,
        }
    }

    /// Intake Stage: Adds a new event, enforcing the hard maximum capacity.
    pub fn intake(&mut self, event: NormalizedTelemetryEvent) {
        if self.events.len() >= self.max_capacity {
            self.events.pop_front(); // Enforce strict bounds
        }
        self.events.push_back(event);
        METRICS.track_active_correlations(self.events.len() as u64);
    }

    /// Cleanup Stage: Removes events older than the longest required rule TTL.
    pub fn cleanup_expired(&mut self, current_time_ms: u64, max_ttl_ms: u64) {
        while let Some(event) = self.events.front() {
            if current_time_ms.saturating_sub(event.timestamp_ms) > max_ttl_ms {
                self.events.pop_front();
            } else {
                break;
            }
        }
        METRICS.track_active_correlations(self.events.len() as u64);
    }
}

/// The 6-Stage Detection Pipeline Orchestrator.
pub struct DetectionPipeline {
    state: CorrelationState,
    rules: Vec<Box<dyn Rule>>,
    max_global_ttl_ms: u64,
}

impl DetectionPipeline {
    pub fn new(rules: Vec<Box<dyn Rule>>) -> Self {
        let max_ttl = rules.iter().map(|r| r.max_correlation_window_ms()).max().unwrap_or(0);
        Self {
            state: CorrelationState::new(50_000), // Hard memory budget
            rules,
            max_global_ttl_ms: max_ttl,
        }
    }

    /// Orchestrates Intake -> Cleanup -> Correlation -> Evaluation -> Risk -> Confidence -> Alert Generation.
    pub fn process_event(&mut self, event: NormalizedTelemetryEvent) -> Vec<Alert> {
        let current_time = event.timestamp_ms;
        
        // 1. Intake
        self.state.intake(event);
        
        // 2. Correlation Cleanup (Bounding state based on Rule TTLs)
        self.state.cleanup_expired(current_time, self.max_global_ttl_ms);
        
        let mut alerts = Vec::new();
        
        // 3. Rule Evaluation (which internally handles Risk and Confidence)
        for rule in &self.rules {
            if let Some(alert) = rule.evaluate(&self.state) {
                // 4, 5, 6. Formatting the immutable alert based on the rule evaluation.
                alerts.push(alert);
            }
        }
        
        alerts
    }
}
