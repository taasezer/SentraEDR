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
        let (_telemetry_tx, _telemetry_rx) = BoundedChannel::new::<sentra_core::TelemetryEvent>(
            self.config.telemetry.channel_capacity,
            "telemetry_stream".to_string(),
        );
        
        let mut ipc_server = sentra_ipc::IpcServer::new()?;
        
        // Spawn the IPC listener loop
        tokio::spawn(async move {
            info!("IPC Named Pipe server starting, waiting for UI connection...");
            let mut sys = sysinfo::System::new_all();
            let start_time = std::time::Instant::now();
            
            loop {
                if let Err(e) = ipc_server.wait_for_client().await {
                    tracing::error!("IPC Server error waiting for client: {}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    continue;
                }
                info!("UI Client connected to IPC pipe");
                
                // Keep connection alive or send health data periodically
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    
                    sys.refresh_all();
                    let cpu_usage = sys.global_cpu_usage();
                    let memory_usage_mb = (sys.used_memory() as f64 / 1024.0 / 1024.0) as f32;
                    let uptime = start_time.elapsed().as_secs();

                    let msg = sentra_ipc::IpcMessage::HealthResponse(sentra_core::SystemHealth {
                        cpu_usage,
                        memory_usage_mb,
                        events_per_second: 0.0, // We will update this when ETW is wired
                        channel_fill_percent: 0.0,
                        dropped_events: 0,
                        uptime_seconds: uptime,
                    });
                    
                    if let Err(e) = ipc_server.send_message(&msg).await {
                        tracing::warn!("IPC client disconnected: {}", e);
                        break;
                    }

                    // Send Process List
                    if let Ok(processes) = sentra_process::enumerate::enumerate_processes() {
                        let proc_msg = sentra_ipc::IpcMessage::ProcessList(processes);
                        if let Err(e) = ipc_server.send_message(&proc_msg).await {
                            tracing::warn!("IPC client disconnected during process list: {}", e);
                            break;
                        }
                    }
                }
                
                // recreate the server instance for next client (named pipes single instance)
                ipc_server = match sentra_ipc::IpcServer::new() {
                    Ok(s) => s,
                    Err(_) => break,
                };
            }
        });

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
