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

        let mut total_events: u64 = 0;
        let mut last_emit = std::time::Instant::now();

        while let Ok(event) = session.receiver.recv() {
            total_events += 1;

            if last_emit.elapsed().as_secs() >= 1 {
                let _ = app_handle.emit("telemetry-stats", total_events);
                last_emit = std::time::Instant::now();
            }

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

fn is_elevated() -> bool {
    unsafe {
        use windows::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
        use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

        let mut token = windows::Win32::Foundation::HANDLE::default();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_ok() {
            let mut elevation = TOKEN_ELEVATION::default();
            let mut size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;
            let result = GetTokenInformation(
                token,
                TokenElevation,
                Some(&mut elevation as *mut _ as *mut _),
                size,
                &mut size,
            );
            if result.is_ok() {
                return elevation.TokenIsElevated != 0;
            }
        }
        false
    }
}

fn run_as_admin() {
    unsafe {
        let exe_path = std::env::current_exe().unwrap();
        let path_u16: Vec<u16> = exe_path.to_string_lossy().encode_utf16().chain(std::iter::once(0)).collect();
        let runas: Vec<u16> = "runas".encode_utf16().chain(std::iter::once(0)).collect();
        
        let _ = windows::Win32::UI::Shell::ShellExecuteW(
            windows::Win32::Foundation::HWND::default(),
            windows::core::PCWSTR(runas.as_ptr()),
            windows::core::PCWSTR(path_u16.as_ptr()),
            windows::core::PCWSTR::null(),
            windows::core::PCWSTR::null(),
            windows::Win32::UI::WindowsAndMessaging::SW_SHOW,
        );
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    if !is_elevated() {
        run_as_admin();
        std::process::exit(0);
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_engine])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
