use sentra_core::SystemHealth;
use sentra_ipc::IpcMessage;
use tauri::{Manager, Emitter};
use tokio::runtime::Runtime;
use sysinfo::System;

#[tauri::command]
fn get_health_status() -> SystemHealth {
    // For now, return mocked health until we receive actual health over IPC
    SystemHealth {
        cpu_usage: 2.4,
        memory_usage_mb: 48.5,
        events_per_second: 120.0,
        channel_fill_percent: 5.0,
        dropped_events: 0,
        uptime_seconds: 3600,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.handle().clone();
            
            // Spawn a background thread for the Monolithic Orchestrator Engine
            std::thread::spawn(move || {
                let rt = Runtime::new().unwrap();
                rt.block_on(async {
                    println!("SentraEDR Monolithic Engine started.");
                    let mut sys = System::new_all();
                    let start_time = std::time::Instant::now();
                    let mut loop_counter = 0;
                    
                    loop {
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        loop_counter += 1;
                        
                        sys.refresh_all();
                        let cpu_usage = sys.global_cpu_info().cpu_usage();
                        let memory_usage_mb = (sys.used_memory() as f64 / 1024.0 / 1024.0) as f32;
                        let uptime = start_time.elapsed().as_secs();

                        // Simulate ETW Live Data Flow (Random events per second between 4000 and 8000)
                        let base_events = 5000.0;
                        let random_fluctuation = (uptime % 10) as f64 * 300.0;
                        let events_per_second = base_events + random_fluctuation;

                        let health_msg = IpcMessage::HealthResponse(SystemHealth {
                            cpu_usage,
                            memory_usage_mb,
                            events_per_second, 
                            channel_fill_percent: 2.5,
                            dropped_events: 0,
                            uptime_seconds: uptime,
                        });
                        
                        // Directly emit to frontend without any IPC pipes!
                        let _ = app_handle.emit("ipc-message", health_msg);

                        // Enumerate and emit process list
                        if let Ok(processes) = sentra_process::enumerate::enumerate_processes() {
                            let proc_msg = IpcMessage::ProcessList(processes);
                            let _ = app_handle.emit("ipc-message", proc_msg);
                        }

                        // Simulate Live Detections every ~8 seconds for the dashboard
                        if loop_counter % 8 == 0 {
                            use sentra_core::{DetectionResult, ThreatLevel, Evidence};
                            use chrono::Utc;
                            use uuid::Uuid;

                            let alert = DetectionResult {
                                id: Uuid::new_v4(),
                                rule_name: "Behavioral.Suspicious.Injection".to_string(),
                                threat_level: ThreatLevel::High,
                                confidence: 0.85,
                                description: "Simulated: Suspicious memory allocation detected in svchost.exe".to_string(),
                                evidence: vec![Evidence {
                                    source: "ETW Memory Monitor".to_string(),
                                    detail: "PAGE_EXECUTE_READWRITE allocated".to_string(),
                                    timestamp: Utc::now(),
                                }],
                                affected_process: None,
                                timestamp: Utc::now(),
                                mitre_technique: Some("T1055".to_string()),
                            };
                            let _ = app_handle.emit("ipc-message", IpcMessage::DetectionAlert(alert));
                        }
                    }
                });
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_health_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
