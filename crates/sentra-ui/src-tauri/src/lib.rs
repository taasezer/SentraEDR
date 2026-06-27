use sentra_core::SystemHealth;
use sentra_ipc::IpcClient;
use tauri::{Manager, Emitter};
use tokio::runtime::Runtime;

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
            
            // Spawn a background thread for the IPC Client
            std::thread::spawn(move || {
                let rt = Runtime::new().unwrap();
                rt.block_on(async {
                    // Try to connect to the SentraEDR IPC Pipe
                    let mut client = match IpcClient::connect().await {
                        Ok(c) => c,
                        Err(e) => {
                            eprintln!("Failed to connect to IPC server: {}", e);
                            return;
                        }
                    };

                    println!("Successfully connected to SentraEDR IPC pipe");

                    loop {
                        match client.receive_message().await {
                            Ok(msg) => {
                                // Emit to frontend
                                let _ = app_handle.emit("ipc-message", msg);
                            }
                            Err(e) => {
                                eprintln!("IPC connection lost: {}", e);
                                break;
                            }
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
