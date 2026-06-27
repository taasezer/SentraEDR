#[test]
fn synthetic_remediation_dry_run_reports_rejection_and_approval_queue() {
    let report = sentra_agent::remediation_dry_run::run_synthetic_remediation_dry_run();

    assert_eq!(report.decisions_evaluated, 2);
    assert_eq!(report.rejected_by_policy, 1);
    assert_eq!(report.waiting_for_approval, 1);
    assert_eq!(report.planned_steps, 3);
}
