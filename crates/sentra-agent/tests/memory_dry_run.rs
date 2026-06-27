#[test]
fn synthetic_memory_analysis_reports_signals() {
    let report = sentra_agent::memory_dry_run::run_synthetic_memory_analysis_dry_run();

    assert_eq!(report.stats.observed, 3);
    assert_eq!(report.stats.handled, 3);
    assert!(
        report
            .signals
            .iter()
            .any(|s| s.name == "remote_thread_creation")
    );
    assert!(
        report
            .signals
            .iter()
            .any(|s| s.name == "executable_private_memory")
    );
    assert!(
        report
            .signals
            .iter()
            .any(|s| s.name == "memory_protection_escalation")
    );
}
