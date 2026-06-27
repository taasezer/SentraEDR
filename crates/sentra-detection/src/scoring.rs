use chrono::{DateTime, Utc};
use sentra_core::{DetectionResult, ThreatLevel};
use std::collections::HashMap;

pub struct ProcessThreatState {
    pub pid: u32,
    pub cumulative_score: f64,
    pub detections: Vec<DetectionResult>,
    pub first_seen: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

pub struct ThreatScorer {
    pub per_process_scores: HashMap<u32, ProcessThreatState>,
}

impl ThreatScorer {
    pub fn new() -> Self {
        Self {
            per_process_scores: HashMap::new(),
        }
    }

    pub fn record_detection(&mut self, pid: u32, result: &DetectionResult) {
        let entry = self.per_process_scores.entry(pid).or_insert(ProcessThreatState {
            pid,
            cumulative_score: 0.0,
            detections: Vec::new(),
            first_seen: Utc::now(),
            last_updated: Utc::now(),
        });

        // Basic scoring logic based on threat level
        let score_add = match result.threat_level {
            ThreatLevel::Critical => 40.0,
            ThreatLevel::High => 20.0,
            ThreatLevel::Medium => 10.0,
            ThreatLevel::Low => 5.0,
            ThreatLevel::None => 0.0,
        };

        entry.cumulative_score += score_add * result.confidence;
        entry.detections.push(result.clone());
        entry.last_updated = Utc::now();
    }

    pub fn get_threat_level(&self, pid: u32) -> ThreatLevel {
        if let Some(state) = self.per_process_scores.get(&pid) {
            let s = state.cumulative_score;
            if s >= 80.0 {
                ThreatLevel::Critical
            } else if s >= 60.0 {
                ThreatLevel::High
            } else if s >= 40.0 {
                ThreatLevel::Medium
            } else if s >= 20.0 {
                ThreatLevel::Low
            } else {
                ThreatLevel::None
            }
        } else {
            ThreatLevel::None
        }
    }

    pub fn decay(&mut self) {
        // Implement time-based score decay
    }

    pub fn cleanup(&mut self) {
        // Implement dead process cleanup
    }
}
