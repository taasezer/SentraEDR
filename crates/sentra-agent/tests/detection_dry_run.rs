use sentra_agent::detection_dry_run::run_synthetic_detection_dry_run;
use shared_models::RiskLevel;

#[test]
fn synthetic_detection_dry_run_reports_observe_only_high_risk_alert() {
    let report = run_synthetic_detection_dry_run();

    assert_eq!(report.stats.signals_observed, 3);
    assert_eq!(report.findings.len(), 1);
    assert_eq!(report.alerts.len(), 1);
    assert_eq!(report.findings[0].risk_level, RiskLevel::High);
    assert!(!report.alerts[0].remediation_eligible);
}
