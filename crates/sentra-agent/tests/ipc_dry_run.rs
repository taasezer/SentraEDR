use sentra_agent::ipc_dry_run::run_synthetic_ipc_dry_run;

#[test]
fn synthetic_ipc_dry_run_routes_health_message() {
    let report = run_synthetic_ipc_dry_run().unwrap();

    assert_eq!(report.stats.chunks_received, 2);
    assert_eq!(report.stats.frames_completed, 1);
    assert_eq!(report.stats.frames_accepted, 1);
    assert_eq!(report.delivered_health_messages, 1);
}
