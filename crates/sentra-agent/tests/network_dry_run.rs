use sentra_agent::network_dry_run::run_synthetic_network_analysis_dry_run;

#[test]
fn synthetic_network_analysis_reports_signals() {
    let report = run_synthetic_network_analysis_dry_run();

    assert_eq!(report.stats.observed, 3);
    assert_eq!(report.stats.handled, 3);
    assert!(
        report
            .signals
            .iter()
            .any(|s| s.name == "beacon_interval_candidate")
    );
    assert!(report.signals.iter().any(|s| s.name == "high_risk_port"));
}
