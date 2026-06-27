use shared_models::{
    Alert, Finding, ProcessIdentity, RiskLevel, Signal, TelemetryEventId, Timestamp,
};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SignalFamily {
    Process,
    Persistence,
    Network,
    PowerShell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalSeverity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectionSignal {
    pub name: String,
    pub description: String,
    pub family: SignalFamily,
    pub severity: SignalSeverity,
    pub timestamp: Timestamp,
    pub confidence: u8,
    pub process: Option<ProcessIdentity>,
    pub supporting_event_ids: Vec<TelemetryEventId>,
}

impl DetectionSignal {
    pub fn new(
        name: impl Into<String>,
        family: SignalFamily,
        severity: SignalSeverity,
        timestamp: Timestamp,
    ) -> Self {
        Self {
            name: name.into(),
            description: String::new(),
            family,
            severity,
            timestamp,
            confidence: 100,
            process: None,
            supporting_event_ids: Vec::new(),
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn with_confidence(mut self, confidence: u8) -> Self {
        self.confidence = confidence.min(100);
        self
    }

    pub fn with_process(mut self, process: ProcessIdentity) -> Self {
        self.process = Some(process);
        self
    }

    pub fn with_event_id(mut self, event_id: TelemetryEventId) -> Self {
        self.supporting_event_ids.push(event_id);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DetectionStats {
    pub signals_observed: u64,
    pub findings_emitted: u64,
    pub alerts_emitted: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectionReport {
    pub stats: DetectionStats,
    pub findings: Vec<Finding>,
    pub alerts: Vec<Alert>,
}

#[derive(Debug, Default)]
pub struct DetectionAnalyzer {
    _private: (),
}

impl DetectionAnalyzer {
    pub fn analyze(&mut self, signals: Vec<DetectionSignal>) -> DetectionReport {
        let mut stats = DetectionStats {
            signals_observed: signals.len() as u64,
            findings_emitted: 0,
            alerts_emitted: 0,
        };

        if signals.is_empty() {
            return DetectionReport {
                stats,
                findings: Vec::new(),
                alerts: Vec::new(),
            };
        }

        let score = score_signals(&signals);
        let risk_level = risk_level(score);
        let timestamp = signals[0].timestamp.clone();
        let mut finding = Finding::new(timestamp, risk_level, score);
        finding.process = signals.iter().find_map(|signal| signal.process.clone());
        finding.telemetry_uncertainty = signals.iter().any(|signal| signal.confidence < 60);
        finding.signals = signals.iter().map(to_shared_signal).collect();
        finding.mitre_techniques = mitre_techniques(&signals);

        let alert = Alert::observe_only(
            finding.clone(),
            "Review correlated observe-only detection before any remediation action",
        );

        stats.findings_emitted = 1;
        stats.alerts_emitted = 1;

        DetectionReport {
            stats,
            findings: vec![finding],
            alerts: vec![alert],
        }
    }
}

fn score_signals(signals: &[DetectionSignal]) -> u8 {
    let base = signals
        .iter()
        .map(|signal| match signal.severity {
            SignalSeverity::Low => 15,
            SignalSeverity::Medium => 35,
            SignalSeverity::High => 55,
        })
        .max()
        .unwrap_or(0);
    let families = signals
        .iter()
        .map(|signal| signal.family)
        .collect::<BTreeSet<_>>()
        .len();
    let diversity_bonus = families.saturating_sub(1) * 15;
    (base + diversity_bonus).min(100) as u8
}

fn risk_level(score: u8) -> RiskLevel {
    match score {
        0..=19 => RiskLevel::Informational,
        20..=39 => RiskLevel::Low,
        40..=69 => RiskLevel::Medium,
        70..=89 => RiskLevel::High,
        _ => RiskLevel::Critical,
    }
}

fn to_shared_signal(signal: &DetectionSignal) -> Signal {
    Signal {
        name: signal.name.clone(),
        description: signal.description.clone(),
        supporting_event_ids: signal.supporting_event_ids.clone(),
    }
}

fn mitre_techniques(signals: &[DetectionSignal]) -> Vec<String> {
    let mut techniques = BTreeSet::new();
    for signal in signals {
        match signal.name.as_str() {
            "powershell_encoded_command" => {
                techniques.insert("T1059.001".to_string());
            }
            "registry_run_key_persistence" => {
                techniques.insert("T1060".to_string());
            }
            "service_persistence" => {
                techniques.insert("T1543.003".to_string());
            }
            "beacon_interval_candidate" => {
                techniques.insert("T1071".to_string());
            }
            _ => {}
        }
    }
    techniques.into_iter().collect()
}
