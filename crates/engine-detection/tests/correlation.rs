use engine_detection::{DetectionAnalyzer, DetectionSignal, SignalFamily, SignalSeverity};
use shared_models::{RiskLevel, Timestamp};

fn ts() -> Timestamp {
    Timestamp::parse_rfc3339("2026-06-27T09:00:00Z").unwrap()
}

fn signal(name: &str, family: SignalFamily, severity: SignalSeverity) -> DetectionSignal {
    DetectionSignal::new(name, family, severity, ts()).with_description(format!("{name} observed"))
}

#[test]
fn multi_family_signals_emit_high_risk_finding_and_observe_only_alert() {
    let mut analyzer = DetectionAnalyzer::default();
    let report = analyzer.analyze(vec![
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
    ]);

    assert_eq!(report.stats.signals_observed, 3);
    assert_eq!(report.findings.len(), 1);
    assert_eq!(report.alerts.len(), 1);
    assert_eq!(report.findings[0].risk_level, RiskLevel::High);
    assert_eq!(report.findings[0].score, 85);
    assert_eq!(report.findings[0].signals.len(), 3);
    assert!(!report.alerts[0].remediation_eligible);
}

#[test]
fn single_medium_signal_emits_low_risk_finding() {
    let mut analyzer = DetectionAnalyzer::default();
    let report = analyzer.analyze(vec![signal(
        "suspicious_dns_pattern",
        SignalFamily::Network,
        SignalSeverity::Medium,
    )]);

    assert_eq!(report.findings[0].risk_level, RiskLevel::Low);
    assert_eq!(report.findings[0].score, 35);
}

#[test]
fn low_confidence_signal_marks_telemetry_uncertainty() {
    let mut analyzer = DetectionAnalyzer::default();
    let report = analyzer.analyze(vec![
        signal(
            "service_persistence",
            SignalFamily::Persistence,
            SignalSeverity::High,
        )
        .with_confidence(40),
    ]);

    assert!(report.findings[0].telemetry_uncertainty);
}
