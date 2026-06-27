use sentra_agent::dry_run::run_synthetic_etw_dry_run;
use shared_models::HealthStatus;

#[test]
fn synthetic_etw_dry_run_reports_two_normalized_events() {
    let report = run_synthetic_etw_dry_run();

    assert_eq!(report.stats.received, 2);
    assert_eq!(report.stats.normalized, 2);
    assert_eq!(report.stats.dropped, 0);
    assert_eq!(report.component_health.status, HealthStatus::Healthy);
}
