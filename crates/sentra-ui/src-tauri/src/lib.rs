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
                    
                    loop {
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        
                        sys.refresh_all();
                        let cpu_usage = sys.global_cpu_info().cpu_usage();
                        let memory_usage_mb = (sys.used_memory() as f64 / 1024.0 / 1024.0) as f32;
                        let uptime = start_time.elapsed().as_secs();

                        let health_msg = IpcMessage::HealthResponse(SystemHealth {
                            cpu_usage,
                            memory_usage_mb,
                            events_per_second: 0.0, // Will be updated by ETW
                            channel_fill_percent: 0.0,
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
                    }
                });
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_health_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
