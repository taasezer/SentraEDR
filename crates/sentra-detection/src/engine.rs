use crate::{EventCorrelator, ThreatScorer, Whitelist};
use sentra_core::{DetectionConfig, DetectionResult, Detector, TelemetryEvent};

pub struct DetectionEngine {
    rules: Vec<Box<dyn Detector + Send + Sync>>,
    correlator: EventCorrelator,
    pub scorer: ThreatScorer,
    whitelist: Whitelist,
}

impl DetectionEngine {
    pub fn new(_config: DetectionConfig) -> Self {
        Self {
            rules: Vec::new(),
            correlator: EventCorrelator::new(),
            scorer: ThreatScorer::new(),
            whitelist: Whitelist::new(),
        }
    }

    pub fn register_rule(&mut self, rule: Box<dyn Detector + Send + Sync>) {
        self.rules.push(rule);
    }

    pub fn analyze(&mut self, event: &TelemetryEvent) -> Vec<DetectionResult> {
        if self.whitelist.is_whitelisted(event) {
            return Vec::new();
        }

        self.correlator.record_event(event);

        let mut results = Vec::new();

        // 1. Single-event rules
        for rule in &self.rules {
            if let Some(detection) = rule.analyze(event) {
                results.push(detection);
            }
        }

        // 2. Correlation rules (simulated here)
        let mut corr_results = self.correlator.correlate(event);
        results.append(&mut corr_results);

        // 3. Score aggregation
        for res in &results {
            if let Some(proc) = &res.affected_process {
                self.scorer.record_detection(proc.pid, res);
            }
        }

        results
    }

    pub fn analyze_batch(&mut self, events: &[TelemetryEvent]) -> Vec<DetectionResult> {
        let mut all_results = Vec::new();
        for e in events {
            all_results.extend(self.analyze(e));
        }
        all_results
    }
}
