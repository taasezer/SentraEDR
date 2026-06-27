use sentra_agent::config::AgentConfig;
use sentra_agent::logging::init_logging;
use sentra_agent::snapshot_builder::build_demo_snapshot;
use tracing::info;

fn main() {
    let config = AgentConfig::default();
    init_logging(&config.logging.level);

    if let Err(error) = config.validate() {
        eprintln!("SentraEDR agent configuration error: {error}");
        std::process::exit(2);
    }

    let snapshot = build_demo_snapshot();

    info!(
        mode = ?config.mode,
        telemetry_capacity = config.queue.telemetry_capacity,
        detection_capacity = config.queue.detection_capacity,
        ipc_enabled = config.ipc.enabled,
        ipc_dispatcher_capacity = snapshot.ipc_dispatcher_capacity,
        etw_received = snapshot.etw_received,
        etw_normalized = snapshot.etw_normalized,
        etw_dropped = snapshot.etw_dropped,
        process_signals = snapshot.process_signals,
        persistence_signals = snapshot.persistence_signals,
        network_signals = snapshot.network_signals,
        memory_signals = snapshot.memory_signals,
        detection_alerts = snapshot.detection_alerts,
        detection_findings = snapshot.detection_findings,
        remediation_decisions = snapshot.remediation_decisions,
        remediation_waiting_approval = snapshot.remediation_waiting_approval,
        remediation_planned_steps = snapshot.remediation_planned_steps,
        ipc_frames_accepted = snapshot.ipc_frames_accepted,
        ipc_frames_failed = snapshot.ipc_frames_failed,
        behavioral_signals = snapshot.total_behavioral_signals(),
        "SentraEDR agent foundation initialized in observe-only mode"
    );
}
