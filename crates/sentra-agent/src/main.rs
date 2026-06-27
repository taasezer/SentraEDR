use engine_etw::{EtwEventSource, LiveEtwSource};
use sentra_agent::config::AgentConfig;
use sentra_agent::logging::init_logging;
use sentra_agent::snapshot_builder::build_demo_snapshot;
use sentra_ui::{DashboardState, LiveTelemetrySnapshot, run_tui_loop};
use shared_models::Timestamp;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

#[tokio::main]
async fn main() {
    let config = AgentConfig::default();
    init_logging(&config.logging.level);

    if let Err(error) = config.validate() {
        eprintln!("SentraEDR agent configuration error: {error}");
        std::process::exit(2);
    }

    // 1. Initialize Dashboard State
    let initial_snapshot = build_demo_snapshot();
    let generated_at = Timestamp::now();
    let mut dashboard = DashboardState::from_alerts(vec![], generated_at);
    let live_snap = LiveTelemetrySnapshot::from_demo_snapshot(&initial_snapshot);
    dashboard.apply_live_telemetry(live_snap);

    let shared_state = Arc::new(RwLock::new(dashboard));

    // 2. Try to initialize Live ETW Source, fallback to Synthetic
    // Since we're bridging LiveEtwSource to our dry-run snapshot for the demo,
    // we'll just run a background loop that updates the dashboard counters
    // when ETW events arrive.
    let etw_source_result = LiveEtwSource::new();
    let state_clone = Arc::clone(&shared_state);

    tokio::spawn(async move {
        match etw_source_result {
            Ok(mut live_source) => {
                info!("Successfully attached to real Windows ETW Kernel-Process provider.");
                loop {
                    // This blocks the thread. In a robust setup we'd use a dedicated OS thread.
                    // But we use a channel receiver so try_recv is fast.
                    match live_source.next_record() {
                        Ok(Some(record)) => {
                            // Update dashboard
                            let mut dash = state_clone.write().await;
                            dash.telemetry.total_received += 1;
                            dash.telemetry.normalized_events += 1;

                            // Let's pretend every new process start is a behavioral signal
                            // to make the demo lively
                            use engine_etw::EtwProcessEventKind;
                            if record.event_kind == EtwProcessEventKind::Start {
                                dash.telemetry.behavioral_signals += 1;
                            }

                            dash.telemetry.last_updated = Timestamp::now();
                        }
                        Ok(None) => {
                            // Sleep a bit so we don't spin 100% CPU when no events
                            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                        }
                        Err(e) => {
                            error!("ETW Stream error: {:?}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                warn!(
                    "Failed to start Live ETW session (are you running as Administrator?). Falling back to synthetic mode. Error: {:?}",
                    e
                );
                // Fallback loop: just increment counters artificially to show the dashboard is "alive"
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    let mut dash = state_clone.write().await;
                    dash.telemetry.total_received += 12;
                    dash.telemetry.normalized_events += 10;
                    dash.telemetry.behavioral_signals += 2;
                    dash.telemetry.last_updated = Timestamp::now();
                }
            }
        }
    });

    // 3. Start Native TUI
    if let Err(e) = run_tui_loop(shared_state).await {
        error!("Terminal UI error: {:?}", e);
    }
}
