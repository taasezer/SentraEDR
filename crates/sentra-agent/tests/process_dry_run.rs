use sentra_agent::process_dry_run::run_synthetic_process_analysis_dry_run;

#[test]
fn synthetic_process_analysis_reports_signals() {
    let report = run_synthetic_process_analysis_dry_run();

    assert_eq!(report.stats.observed, 2);
    assert_eq!(report.stats.started, 2);
    assert_eq!(report.stats.exited, 0);
    assert_eq!(report.stats.ignored, 0);
    assert_eq!(report.tracked_processes, 2);
    assert_eq!(report.signals.len(), 2);
    assert!(
        report
            .signals
            .iter()
            .any(|signal| signal.name == "suspicious_parent_child")
    );
    assert!(
        report
            .signals
            .iter()
            .any(|signal| signal.name == "powershell_encoded_command")
    );
}
