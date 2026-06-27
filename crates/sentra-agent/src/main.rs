use sentra_agent::config::AgentConfig;
use sentra_agent::dry_run::run_synthetic_etw_dry_run;
use sentra_agent::logging::init_logging;
use tracing::info;

fn main() {
    let config = AgentConfig::default();
    init_logging(&config.logging.level);

    if let Err(error) = config.validate() {
        eprintln!("SentraEDR agent configuration error: {error}");
        std::process::exit(2);
    }

    let report = run_synthetic_etw_dry_run();

    info!(
        mode = ?config.mode,
        telemetry_capacity = config.queue.telemetry_capacity,
        detection_capacity = config.queue.detection_capacity,
        etw_received = report.stats.received,
        etw_normalized = report.stats.normalized,
        etw_dropped = report.stats.dropped,
        "SentraEDR agent foundation initialized in observe-only mode"
    );
}
