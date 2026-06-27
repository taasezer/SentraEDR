use sentra_core::SystemHealth;

#[tauri::command]
fn get_health_status() -> SystemHealth {
    // For now, return mocked health. Later this will connect to sentra_ipc!
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
        .invoke_handler(tauri::generate_handler![get_health_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
