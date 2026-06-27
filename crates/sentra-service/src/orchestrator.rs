use sentra_core::{Result, SentraConfig};
use sentra_detection::DetectionEngine;
use sentra_ipc::bus::EventBus;
use sentra_ipc::channel::BoundedChannel;
use sentra_remediation::RemediationEngine;
use tokio::signal;
use tracing::info;

pub struct SentraOrchestrator {
    config: SentraConfig,
}

impl SentraOrchestrator {
    pub fn new(config: SentraConfig) -> Self {
        Self { config }
    }

    pub async fn run(&mut self) -> Result<()> {
        info!("Initializing SentraEDR Orchestrator...");

        // 1. Setup IPC and Event Bus
        let event_bus = EventBus::new(self.config.telemetry.channel_capacity);
        let (_telemetry_tx, _telemetry_rx) = BoundedChannel::new(
            self.config.telemetry.channel_capacity,
            "telemetry_stream".to_string(),
        );

        // 2. Initialize Telemetry Pipeline
        // In a real implementation we would spawn the ETW consumer here
        info!("Starting telemetry pipeline...");

        // 3. Initialize Detection Engine
        let _detection_engine = DetectionEngine::new(self.config.detection.clone());
        info!("Starting detection engine...");

        // 4. Initialize Remediation Engine
        let _remediation_engine = RemediationEngine::new(self.config.remediation.clone());
        info!("Starting remediation engine...");

        // 5. Initialize Subsystems (Network, Process, Persistence)
        info!("Starting monitoring subsystems...");

        // Wait for shutdown signal
        info!("SentraEDR running. Press Ctrl+C to stop.");
        match signal::ctrl_c().await {
            Ok(()) => info!("Shutdown signal received. Stopping gracefully..."),
            Err(err) => tracing::error!("Unable to listen for shutdown signal: {}", err),
        }

        // Cleanup logic would go here

        Ok(())
    }
}
