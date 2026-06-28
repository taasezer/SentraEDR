use engine_etw::EtwSession;
use engine_detection::pipeline::DetectionPipeline;
use engine_detection::rules::{LsassDumpRule, ReverseShellRule, RansomwareBehaviorRule, ProcessInjectionRule, RegistryPersistenceRule};
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
            Box::new(ProcessInjectionRule),
            Box::new(RegistryPersistenceRule),
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
                
                use tauri_plugin_notification::NotificationExt;
                let _ = app_handle.notification()
                    .builder()
                    .title(format!("SentraEDR: {}", alert.rule_id))
                    .body(format!("Threat detected! Severity: {}", alert.severity))
                    .show();

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
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            use tauri::Manager;
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            use tauri::menu::{Menu, MenuItem};
            use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};

            let show_i = MenuItem::with_id(app, "show", "Toggle Dashboard", true, None::<&str>)?;
            let disable_i = MenuItem::with_id(app, "disable", "Disable Protection", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit SentraEDR", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[&show_i, &disable_i, &quit_i])?;

            let _tray = TrayIconBuilder::with_id("sentra-tray")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        println!("Quitting SentraEDR...");
                        let _ = app.emit("engine-error", "Engine Shutting Down");
                        std::process::exit(0);
                    },
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                    "disable" => {
                        let _ = app.emit("engine-error", "Protection Disabled by User");
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } => {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                let _ = window.hide();
                api.prevent_close();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![start_engine])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
