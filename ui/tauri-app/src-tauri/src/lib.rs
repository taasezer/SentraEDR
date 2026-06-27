use engine_etw::EtwSession;
use engine_detection::pipeline::DetectionPipeline;
use engine_detection::rules::{LsassDumpRule, ReverseShellRule, RansomwareBehaviorRule};
use tauri::{Manager, Emitter};

#[tauri::command]
fn start_engine(app_handle: tauri::AppHandle) {
    // Spawn the Engine in a detached thread to prevent blocking the UI
    std::thread::spawn(move || {
        println!("Starting SentraEDR Embedded Engine...");

        let session = match EtwSession::start_trace() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to start ETW trace (Are you running as Administrator?): {}", e);
                let _ = app_handle.emit("engine-error", format!("ETW Engine Failed: {}", e));
                return;
            }
        };

        let _ = app_handle.emit("engine-started", "Active");

        let rules: Vec<Box<dyn engine_detection::rules::Rule>> = vec![
            Box::new(LsassDumpRule),
            Box::new(ReverseShellRule),
            Box::new(RansomwareBehaviorRule),
        ];
        let mut detection_engine = DetectionPipeline::new(rules);

        println!("Embedded Engine initialized. Waiting for telemetry...");

        while let Ok(event) = session.receiver.recv() {
            let alerts = detection_engine.process_event(event);
            for alert in alerts {
                println!("[ALERT] {}", alert.rule_id);
                if let Ok(alert_json) = serde_json::to_string(&alert) {
                    let _ = app_handle.emit("edr-alert", alert_json);
                }
            }
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_engine])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
