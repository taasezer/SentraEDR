use sentra_agent::config::AgentConfig;
use sentra_agent::dry_run::run_synthetic_etw_dry_run;
use sentra_agent::logging::init_logging;
use sentra_agent::process_dry_run::run_synthetic_process_analysis_dry_run;
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

    info!(
        mode = ?config.mode,
        telemetry_capacity = config.queue.telemetry_capacity,
        detection_capacity = config.queue.detection_capacity,
        etw_received = etw_report.stats.received,
        etw_normalized = etw_report.stats.normalized,
        etw_dropped = etw_report.stats.dropped,
        process_observed = process_report.stats.observed,
        process_started = process_report.stats.started,
        process_signals = process_report.signals.len(),
        process_tracked = process_report.tracked_processes,
        "SentraEDR agent foundation initialized in observe-only mode"
    );
}
