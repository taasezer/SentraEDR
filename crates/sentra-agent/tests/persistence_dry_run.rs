use sentra_agent::persistence_dry_run::run_synthetic_persistence_analysis_dry_run;

#[test]
fn synthetic_persistence_analysis_reports_signals() {
    let report = run_synthetic_persistence_analysis_dry_run();

    assert_eq!(report.stats.observed, 2);
    assert_eq!(report.stats.handled, 2);
    assert_eq!(report.stats.ignored, 0);
    assert_eq!(report.signals.len(), 2);
    assert!(
        report
            .signals
            .iter()
            .any(|signal| signal.name == "registry_run_key_persistence")
    );
    assert!(
        report
            .signals
            .iter()
            .any(|signal| signal.name == "service_persistence")
    );
}
