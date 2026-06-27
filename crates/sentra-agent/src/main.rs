use sentra_agent::config::AgentConfig;
use sentra_agent::logging::init_logging;
use tracing::info;

fn main() {
    let config = AgentConfig::default();
    init_logging(&config.logging.level);

    if let Err(error) = config.validate() {
        eprintln!("SentraEDR agent configuration error: {error}");
        std::process::exit(2);
    }

    info!(
        mode = ?config.mode,
        telemetry_capacity = config.queue.telemetry_capacity,
        detection_capacity = config.queue.detection_capacity,
        "SentraEDR agent foundation initialized in observe-only mode"
    );
}
