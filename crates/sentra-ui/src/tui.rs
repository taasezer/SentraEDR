use crate::DashboardState;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
};
use std::{io, sync::Arc, time::Duration};
use tokio::sync::RwLock;

pub type SharedDashboardState = Arc<RwLock<DashboardState>>;

pub async fn run_tui_loop(state: SharedDashboardState) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        let dashboard = state.read().await.clone();

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3), // Header
                        Constraint::Length(5), // Core Metrics
                        Constraint::Length(5), // IPC Metrics
                        Constraint::Min(0),    // Risk & Timeline
                    ]
                    .as_ref(),
                )
                .split(f.size());

            // 1. Header
            let header = Paragraph::new(format!(
                " SentraEDR Native Terminal Dashboard | Press 'q' to exit | Mode: Observe-only | Last Updated: {}",
                dashboard.telemetry.last_updated.to_rfc3339()
            ))
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL).title("Status"));
            f.render_widget(header, chunks[0]);

            // 2. Core Metrics (Horizontal Split)
            let core_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                ])
                .split(chunks[1]);

            let total_events = dashboard.telemetry.total_received;
            let normalized = dashboard.telemetry.normalized_events;
            
            let normalized_pct = if total_events > 0 {
                (normalized as f64 / total_events as f64 * 100.0) as u16
            } else {
                0
            };

            f.render_widget(
                Gauge::default()
                    .block(Block::default().borders(Borders::ALL).title("Events Ingested"))
                    .gauge_style(Style::default().fg(Color::Blue))
                    .percent(100)
                    .label(total_events.to_string()),
                core_chunks[0],
            );

            f.render_widget(
                Gauge::default()
                    .block(Block::default().borders(Borders::ALL).title("Normalized"))
                    .gauge_style(Style::default().fg(Color::Green))
                    .percent(normalized_pct.min(100))
                    .label(format!("{} ({}%)", normalized, normalized_pct)),
                core_chunks[1],
            );

            let behavioral = dashboard.telemetry.behavioral_signals;
            let behavioral_pct = if normalized > 0 {
                (behavioral as f64 / normalized as f64 * 100.0) as u16
            } else {
                0
            };

            f.render_widget(
                Gauge::default()
                    .block(Block::default().borders(Borders::ALL).title("Behavioral Signals"))
                    .gauge_style(Style::default().fg(Color::Magenta))
                    .percent(behavioral_pct.min(100))
                    .label(behavioral.to_string()),
                core_chunks[2],
            );

            let alerts = dashboard.telemetry.detection_alerts;
            f.render_widget(
                Paragraph::new(alerts.to_string())
                    .style(if alerts > 0 { Style::default().fg(Color::Red).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::White) })
                    .block(Block::default().borders(Borders::ALL).title("Detection Alerts")),
                core_chunks[3],
            );

            // 3. IPC & Health
            let ipc_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)])
                .split(chunks[2]);

            let dropped = dashboard.telemetry.dropped_events;
            f.render_widget(
                Paragraph::new(dropped.to_string())
                    .style(if dropped > 0 { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::White) })
                    .block(Block::default().borders(Borders::ALL).title("Dropped Events")),
                ipc_chunks[0],
            );

            f.render_widget(
                Paragraph::new(format!("Accepted: {} | Failed: {}", dashboard.telemetry.ipc_frames_accepted, dashboard.telemetry.ipc_failed_frames))
                    .block(Block::default().borders(Borders::ALL).title("IPC Dispatcher")),
                ipc_chunks[1],
            );

            f.render_widget(
                Paragraph::new(format!("{:?}", dashboard.telemetry.agent_status))
                    .style(Style::default().fg(Color::Green))
                    .block(Block::default().borders(Borders::ALL).title("Agent Health")),
                ipc_chunks[2],
            );

            // 4. Lower Area (Risk Summary & Timeline)
            let lower_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                .split(chunks[3]);

            let summary = dashboard.summary;
            let risk_text = vec![
                Line::from(vec![Span::raw(format!("Total Alerts: {}", summary.total_alerts))]),
                Line::from(vec![Span::styled(format!("Critical: {}", summary.critical), Style::default().fg(Color::Red))]),
                Line::from(vec![Span::styled(format!("High: {}", summary.high), Style::default().fg(Color::LightRed))]),
                Line::from(vec![Span::styled(format!("Medium: {}", summary.medium), Style::default().fg(Color::Yellow))]),
                Line::from(vec![Span::styled(format!("Low: {}", summary.low), Style::default().fg(Color::Green))]),
            ];

            f.render_widget(
                Paragraph::new(risk_text)
                    .block(Block::default().borders(Borders::ALL).title("Risk Summary")),
                lower_chunks[0],
            );

            // Timeline
            let mut timeline_lines = Vec::new();
            for entry in dashboard.timeline.iter().rev().take(15) {
                timeline_lines.push(Line::from(vec![
                    Span::styled(format!("[{}] ", entry.timestamp.to_rfc3339()), Style::default().fg(Color::DarkGray)),
                    Span::styled(format!("{:?} ", entry.kind), Style::default().fg(Color::Cyan)),
                    Span::raw(entry.title.clone()),
                ]));
            }

            f.render_widget(
                Paragraph::new(timeline_lines)
                    .block(Block::default().borders(Borders::ALL).title("Event Timeline")),
                lower_chunks[1],
            );
        })?;

        // Poll for input gracefully
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char('k') => {
                            // Demo: Spawn and Kill process using our Executor
                            use std::process::Command;
                            if let Ok(child) = Command::new("notepad.exe").spawn() {
                                let pid = child.id();
                                
                                use engine_remediation::{RemediationPolicy, RemediationEngine, executor::RemediationExecutor};
                                use shared_models::{Alert, Finding, RemediationAction, RiskLevel, ProcessIdentity, Timestamp};

                                let mut finding = Finding::new(
                                    Timestamp::now(),
                                    RiskLevel::Critical,
                                    100
                                );
                                
                                finding.process = Some(ProcessIdentity {
                                    process_id: pid,
                                    parent_process_id: None,
                                    image_path: None,
                                    command_line: None,
                                    user_sid: None,
                                });

                                let mut alert = Alert::observe_only(
                                    finding,
                                    "Simulated finding to test process termination",
                                );
                                alert.remediation_eligible = true;
                                
                                let policy = RemediationPolicy::approval_required()
                                    .with_allowed_actions(vec![RemediationAction::KillProcess]);
                                    
                                let engine = RemediationEngine::new(policy);
                                let decision = engine.evaluate(&alert, Timestamp::now());
                                
                                if let Some(plan) = decision.plan {
                                    if RemediationExecutor::execute_plan(&plan, &alert).is_ok() {
                                        let mut dash = state.write().await;
                                        use crate::{TimelineEntry, TimelineKind};
                                        dash.timeline.push(TimelineEntry {
                                            kind: TimelineKind::ActionQueued,
                                            title: format!("TERMINATED PROCESS: PID {} (Notepad)", pid),
                                            timestamp: Timestamp::now(),
                                        });
                                    }
                                }
                            }
                        }
                        KeyCode::Char('m') => {
                            // Demo: Scan our own memory or a specific process
                            let pid = std::process::id();
                            use engine_memory::scanner::MemoryScanner;
                            use crate::{TimelineEntry, TimelineKind};
                            use shared_models::Timestamp;

                            match MemoryScanner::scan_process(pid) {
                                Ok(regions) => {
                                    let mut dash = state.write().await;
                                    if regions.is_empty() {
                                        dash.timeline.push(TimelineEntry {
                                            kind: TimelineKind::TelemetryUpdated,
                                            title: format!("MEMORY SCAN: PID {} is clean.", pid),
                                            timestamp: Timestamp::now(),
                                        });
                                    } else {
                                        dash.timeline.push(TimelineEntry {
                                            kind: TimelineKind::AlertObserved,
                                            title: format!("MEMORY SCAN: PID {} has {} suspicious unbacked regions!", pid, regions.len()),
                                            timestamp: Timestamp::now(),
                                        });
                                    }
                                }
                                Err(e) => {
                                    let mut dash = state.write().await;
                                    dash.timeline.push(TimelineEntry {
                                        kind: TimelineKind::AlertObserved,
                                        title: format!("MEMORY SCAN FAILED: {:?}", e),
                                        timestamp: Timestamp::now(),
                                    });
                                }
                            }
                        }
                        KeyCode::Char('p') => {
                            // Demo: Scan persistence (Registry Run keys)
                            use engine_persistence::scanner::PersistenceScanner;
                            use crate::{TimelineEntry, TimelineKind};
                            use shared_models::Timestamp;

                            if let Ok(entries) = PersistenceScanner::scan_run_keys() {
                                let mut dash = state.write().await;
                                if entries.is_empty() {
                                    dash.timeline.push(TimelineEntry {
                                        kind: TimelineKind::TelemetryUpdated,
                                        title: "PERSISTENCE SCAN: No suspicious auto-run entries found.".to_string(),
                                        timestamp: Timestamp::now(),
                                    });
                                } else {
                                    dash.timeline.push(TimelineEntry {
                                        kind: TimelineKind::AlertObserved,
                                        title: format!("PERSISTENCE SCAN: Found {} suspicious Registry Run keys!", entries.len()),
                                        timestamp: Timestamp::now(),
                                    });
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
