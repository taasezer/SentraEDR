use engine_detection::{
    DetectionAnalyzer, DetectionReport, DetectionSignal, SignalFamily, SignalSeverity,
};
use shared_models::Timestamp;

pub fn run_synthetic_detection_dry_run() -> DetectionReport {
    let mut analyzer = DetectionAnalyzer::default();
    analyzer.analyze(vec![
        signal(
            "powershell_encoded_command",
            SignalFamily::Process,
            SignalSeverity::Medium,
        ),
        signal(
            "registry_run_key_persistence",
            SignalFamily::Persistence,
            SignalSeverity::High,
        ),
        signal(
            "beacon_interval_candidate",
            SignalFamily::Network,
            SignalSeverity::High,
        ),
    ])
}

fn signal(name: &str, family: SignalFamily, severity: SignalSeverity) -> DetectionSignal {
    DetectionSignal::new(
        name,
        family,
        severity,
        Timestamp::parse_rfc3339("2026-06-27T09:07:00Z").unwrap(),
    )
    .with_description(format!("{name} observed in synthetic dry run"))
}
