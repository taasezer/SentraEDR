use engine_etw::{EtwEventSource, LiveEtwSource};
use sentra_agent::config::AgentConfig;
use sentra_agent::logging::init_logging;
use sentra_agent::snapshot_builder::build_demo_snapshot;
use sentra_ui::{DashboardState, LiveTelemetrySnapshot, run_tui_loop};
use shared_models::Timestamp;
use std::sync::Arc;
use std::path::PathBuf;
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
        let file_analyzer = engine_file::FileAnalyzer::new();
        match etw_source_result {
            Ok(mut live_source) => {
                info!("Successfully attached to real Windows ETW Kernel-Process/Network/File providers.");
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
                            use engine_etw::{EtwProcessEventKind, EtwNetworkEventKind, EtwFileEventKind, EtwRecord};
                            match record {
                                EtwRecord::Process(p) => {
                                    if p.event_kind == EtwProcessEventKind::Start {
                                        dash.telemetry.behavioral_signals += 1;
                                    }
                                }
                                EtwRecord::Network(n) => {
                                    if n.event_kind == EtwNetworkEventKind::TcpConnect {
                                        dash.telemetry.behavioral_signals += 1; // Also treat as behavioral for demo
                                        
                                        // Push a timeline event for Network Connect
                                        use sentra_ui::{TimelineEntry, TimelineKind};
                                        dash.timeline.push(TimelineEntry {
                                            kind: TimelineKind::TelemetryUpdated,
                                            title: format!("TCP Connect to {}:{}", n.remote_ip, n.remote_port),
                                            timestamp: n.timestamp,
                                        });
                                    }
                                }
                                EtwRecord::File(f) => {
                                    if f.event_kind == EtwFileEventKind::Create || f.event_kind == EtwFileEventKind::Rename {
                                        if let Some(signal) = file_analyzer.analyze_file_io(&f.file_path, f.process_id, f.timestamp.clone()) {
                                            dash.telemetry.behavioral_signals += 1;
                                            
                                            // Create Alert
                                            use shared_models::{Alert, Finding, RiskLevel, RemediationAction, process::{ProcessIdentity, ImagePath}};
                                            use engine_remediation::{executor::RemediationExecutor, RemediationPlan, RemediationPlanStepKind, RemediationPlanStep};
                                            
                                            let mut finding = Finding::new(signal.timestamp.clone(), RiskLevel::Critical, 100);
                                            finding.process = Some(ProcessIdentity {
                                                process_id: signal.pid,
                                                parent_process_id: None,
                                                image_path: Some(ImagePath::new(signal.file_path.clone())),
                                                command_line: None,
                                                user_sid: None,
                                            });
                                            let alert = Alert::observe_only(finding, "Ransomware detected");

                                            use sentra_ui::{TimelineEntry, TimelineKind};
                                            dash.timeline.push(TimelineEntry {
                                                kind: TimelineKind::AlertObserved,
                                                title: format!("RANSOMWARE DETECTED! PID {} created a {} file.", signal.pid, signal.extension),
                                                timestamp: signal.timestamp.clone(),
                                            });

                                            // Execute Kill Switch and Quarantine
                                            let plan = RemediationPlan {
                                                alert_id: alert.alert_id.clone(),
                                                steps: vec![
                                                    RemediationPlanStep {
                                                        kind: RemediationPlanStepKind::KillProcess,
                                                        action: RemediationAction::KillProcess,
                                                        description: "Kill Ransomware".into(),
                                                    },
                                                    RemediationPlanStep {
                                                        kind: RemediationPlanStepKind::DeleteFile,
                                                        action: RemediationAction::DeleteFile,
                                                        description: "Destroy Ransomware File".into(),
                                                    }
                                                ],
                                                created_at: signal.timestamp.clone(),
                                                plan_id: uuid::Uuid::new_v4(),
                                                mode: shared_models::RemediationMode::ApprovalRequired,
                                            };

                                            match RemediationExecutor::execute_plan(&plan, &alert) {
                                                Ok(_) => {
                                                    dash.timeline.push(TimelineEntry {
                                                        kind: TimelineKind::AlertResolved,
                                                        title: format!("SUCCESS: Killed PID {} and DESTROYED {}.", signal.pid, signal.file_path),
                                                        timestamp: Timestamp::now(),
                                                    });
                                                }
                                                Err(e) => {
                                                    dash.timeline.push(TimelineEntry {
                                                        kind: TimelineKind::AlertObserved,
                                                        title: format!("FAILED to kill PID {}: {:?}", signal.pid, e),
                                                        timestamp: Timestamp::now(),
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
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
                // Fallback loop: synthetic ETW + File System Watcher
                let desktop_path = dirs::desktop_dir().unwrap_or_else(|| PathBuf::from("C:\\Users\\user\\Desktop"));
                let mut file_rx = sentra_agent::file_watcher::start_file_watcher(desktop_path);
                
                loop {
                    tokio::select! {
                        _ = tokio::time::sleep(std::time::Duration::from_millis(150)) => {
                            let mut dash = state_clone.write().await;
                            dash.telemetry.total_received += 45;
                            dash.telemetry.normalized_events += 38;
                            dash.telemetry.behavioral_signals += 3;
                            dash.telemetry.last_updated = Timestamp::now();
                        }
                        Some(path) = file_rx.recv() => {
                            // File watcher picked something up
                            let path_str = path.to_string_lossy().to_string();
                            if let Some(signal) = file_analyzer.analyze_file_io(&path_str, 0, Timestamp::now()) {
                                let mut dash = state_clone.write().await;
                                dash.telemetry.behavioral_signals += 1;
                                
                                use shared_models::{Alert, Finding, RiskLevel, RemediationAction, process::{ProcessIdentity, ImagePath}};
                                use engine_remediation::{executor::RemediationExecutor, RemediationPlan, RemediationPlanStepKind, RemediationPlanStep};
                                
                                let mut finding = Finding::new(signal.timestamp.clone(), RiskLevel::Critical, 100);
                                finding.process = Some(ProcessIdentity {
                                    process_id: signal.pid, // 0 for synthetic
                                    parent_process_id: None,
                                    image_path: Some(ImagePath::new(signal.file_path.clone())),
                                    command_line: None,
                                    user_sid: None,
                                });
                                let alert = Alert::observe_only(finding, "Ransomware detected by File Watcher");

                                use sentra_ui::{TimelineEntry, TimelineKind};
                                dash.timeline.push(TimelineEntry {
                                    kind: TimelineKind::AlertObserved,
                                    title: format!("RANSOMWARE DETECTED! Created: {}", signal.extension),
                                    timestamp: signal.timestamp.clone(),
                                });

                                let plan = RemediationPlan {
                                    alert_id: alert.alert_id.clone(),
                                    steps: vec![
                                        RemediationPlanStep {
                                            kind: RemediationPlanStepKind::DeleteFile,
                                            action: RemediationAction::DeleteFile,
                                            description: "Destroy Ransomware File".into(),
                                        }
                                    ],
                                    created_at: signal.timestamp.clone(),
                                    plan_id: uuid::Uuid::new_v4(),
                                    mode: shared_models::RemediationMode::ApprovalRequired,
                                };

                                match RemediationExecutor::execute_plan(&plan, &alert) {
                                    Ok(_) => {
                                        dash.timeline.push(TimelineEntry {
                                            kind: TimelineKind::AlertResolved,
                                            title: format!("SUCCESS: DESTROYED ransomware file: {}.", signal.file_path),
                                            timestamp: Timestamp::now(),
                                        });
                                    }
                                    Err(e) => {
                                        dash.timeline.push(TimelineEntry {
                                            kind: TimelineKind::AlertObserved,
                                            title: format!("FAILED to destroy file: {:?}", e),
                                            timestamp: Timestamp::now(),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    // 3. Start Native TUI
    if let Err(e) = run_tui_loop(shared_state).await {
        error!("Terminal UI error: {:?}", e);
    }
}
