use sentra_agent::config::AgentConfig;
use sentra_agent::detection_dry_run::run_synthetic_detection_dry_run;
use sentra_agent::dry_run::run_synthetic_etw_dry_run;
use sentra_agent::ipc_dry_run::run_synthetic_ipc_dry_run;
use sentra_agent::logging::init_logging;
use sentra_agent::memory_dry_run::run_synthetic_memory_analysis_dry_run;
use sentra_agent::network_dry_run::run_synthetic_network_analysis_dry_run;
use sentra_agent::persistence_dry_run::run_synthetic_persistence_analysis_dry_run;
use sentra_agent::process_dry_run::run_synthetic_process_analysis_dry_run;
use sentra_agent::remediation_dry_run::run_synthetic_remediation_dry_run;
use tracing::info;

fn main() {
    let config = AgentConfig::default();
    init_logging(&config.logging.level);

    if let Err(error) = config.validate() {
        eprintln!("SentraEDR agent configuration error: {error}");
        std::process::exit(2);
    }

    let etw_report = run_synthetic_etw_dry_run();
    let process_report = run_synthetic_process_analysis_dry_run();
    let persistence_report = run_synthetic_persistence_analysis_dry_run();
    let network_report = run_synthetic_network_analysis_dry_run();
    let memory_report = run_synthetic_memory_analysis_dry_run();
    let detection_report = run_synthetic_detection_dry_run();
    let remediation_report = run_synthetic_remediation_dry_run();
    let ipc_report = run_synthetic_ipc_dry_run().unwrap_or_else(|error| {
        eprintln!("SentraEDR agent IPC dry-run error: {error}");
        std::process::exit(2);
    });

    info!(
        mode = ?config.mode,
        telemetry_capacity = config.queue.telemetry_capacity,
        detection_capacity = config.queue.detection_capacity,
        ipc_enabled = config.ipc.enabled,
        ipc_dispatcher_capacity = config.ipc.dispatcher_capacity,
        etw_received = etw_report.stats.received,
        etw_normalized = etw_report.stats.normalized,
        etw_dropped = etw_report.stats.dropped,
        process_observed = process_report.stats.observed,
        process_started = process_report.stats.started,
        process_signals = process_report.signals.len(),
        process_tracked = process_report.tracked_processes,
        persistence_observed = persistence_report.stats.observed,
        persistence_handled = persistence_report.stats.handled,
        persistence_signals = persistence_report.signals.len(),
        network_observed = network_report.stats.observed,
        network_handled = network_report.stats.handled,
        network_signals = network_report.signals.len(),
        memory_observed = memory_report.stats.observed,
        memory_handled = memory_report.stats.handled,
        memory_signals = memory_report.signals.len(),
        detection_signals = detection_report.stats.signals_observed,
        detection_findings = detection_report.findings.len(),
        detection_alerts = detection_report.alerts.len(),
        remediation_decisions = remediation_report.decisions_evaluated,
        remediation_rejected = remediation_report.rejected_by_policy,
        remediation_waiting_for_approval = remediation_report.waiting_for_approval,
        remediation_planned_steps = remediation_report.planned_steps,
        ipc_chunks = ipc_report.stats.chunks_received,
        ipc_frames_completed = ipc_report.stats.frames_completed,
        ipc_frames_accepted = ipc_report.stats.frames_accepted,
        ipc_stream_rejected = ipc_report.stats.stream_rejected,
        ipc_decode_failed = ipc_report.stats.intake_decode_failed,
        ipc_dispatch_failed = ipc_report.stats.intake_dispatch_failed,
        ipc_health_messages = ipc_report.delivered_health_messages,
        "SentraEDR agent foundation initialized in observe-only mode"
    );
}
