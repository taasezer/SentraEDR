use crate::{DashboardState, TimelineKind};
use shared_models::{HealthStatus, RemediationAction, RiskLevel};
use std::fmt::Write;

pub fn render_dashboard_html(dashboard: &DashboardState) -> String {
    let mut html = String::new();
    html.push_str("<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\">");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">");
    html.push_str("<title>SentraEDR Demo Dashboard</title>");
    html.push_str("<link rel=\"preconnect\" href=\"https://fonts.googleapis.com\">");
    html.push_str("<link rel=\"preconnect\" href=\"https://fonts.gstatic.com\" crossorigin>");
    html.push_str("<link href=\"https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&display=swap\" rel=\"stylesheet\">");
    html.push_str("<style>");
    push_css(&mut html);
    html.push_str("</style></head><body><main>");

    render_header(&mut html, dashboard);
    render_telemetry_grid(&mut html, dashboard);
    render_lower_grid(&mut html, dashboard);
    render_footer(&mut html, dashboard);

    html.push_str("</main>");

    // JS to auto-refresh the main content every 2 seconds without full page reload
    html.push_str(
        r#"<script>
        setInterval(async () => {
            try {
                let res = await fetch("/");
                if (!res.ok) return;
                let text = await res.text();
                let parser = new DOMParser();
                let doc = parser.parseFromString(text, "text/html");
                let newMain = doc.querySelector("main");
                if (newMain) {
                    document.querySelector("main").innerHTML = newMain.innerHTML;
                }
            } catch (err) {
                console.error("Live telemetry fetch failed", err);
            }
        }, 2000);
        </script>"#,
    );

    html.push_str("</body></html>");
    html
}

fn push_css(html: &mut String) {
    html.push_str(
        r#"*,*::before,*::after{box-sizing:border-box;margin:0;padding:0}
body{background:#0f1117;color:#e2e8f0;font-family:'Inter',system-ui,-apple-system,sans-serif;-webkit-font-smoothing:antialiased}
main{max-width:1240px;margin:0 auto;padding:32px 24px}
@keyframes fadeIn{from{opacity:0;transform:translateY(8px)}to{opacity:1;transform:translateY(0)}}
@keyframes pulse{0%,100%{opacity:1}50%{opacity:.6}}
header{display:flex;align-items:flex-start;justify-content:space-between;gap:16px;margin-bottom:28px;animation:fadeIn .5s ease-out}
.eyebrow{font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:1.5px;color:#7c8ca1;margin-bottom:6px}
.title{font-size:30px;font-weight:800;line-height:1.15;background:linear-gradient(135deg,#60a5fa,#a78bfa);-webkit-background-clip:text;-webkit-text-fill-color:transparent;background-clip:text}
.subtitle{color:#64748b;font-size:14px;margin-top:6px;line-height:1.5}
.status-badge{display:inline-flex;align-items:center;gap:8px;border:1px solid rgba(255,255,255,.08);border-radius:12px;padding:10px 16px;background:rgba(255,255,255,.04);backdrop-filter:blur(12px);font-weight:600;font-size:14px}
.dot{width:10px;height:10px;border-radius:50%;flex-shrink:0}
.dot.healthy{background:#22c55e;box-shadow:0 0 8px rgba(34,197,94,.5);animation:pulse 2s ease-in-out infinite}
.dot.degraded{background:#f59e0b;box-shadow:0 0 8px rgba(245,158,11,.5);animation:pulse 2s ease-in-out infinite}
.dot.stopped{background:#64748b}
.grid{display:grid;grid-template-columns:repeat(4,minmax(0,1fr));gap:14px;margin-bottom:18px}
.panel{background:rgba(255,255,255,.03);border:1px solid rgba(255,255,255,.06);border-radius:14px;padding:20px;transition:border-color .2s,box-shadow .2s,transform .2s;animation:fadeIn .5s ease-out backwards}
.panel:hover{border-color:rgba(255,255,255,.12);box-shadow:0 4px 24px rgba(0,0,0,.3);transform:translateY(-2px)}
.panel:nth-child(1){animation-delay:.05s}.panel:nth-child(2){animation-delay:.1s}.panel:nth-child(3){animation-delay:.15s}.panel:nth-child(4){animation-delay:.2s}
.panel:nth-child(5){animation-delay:.25s}.panel:nth-child(6){animation-delay:.3s}.panel:nth-child(7){animation-delay:.35s}.panel:nth-child(8){animation-delay:.4s}
.panel h2{font-size:13px;font-weight:600;color:#7c8ca1;text-transform:uppercase;letter-spacing:.5px;margin-bottom:14px}
.metric{font-size:32px;font-weight:800;line-height:1;color:#f1f5f9}
.label{color:#64748b;font-size:12px;margin-top:6px;line-height:1.4}
.wide{grid-column:span 2}
.bar-track{height:6px;background:rgba(255,255,255,.06);border-radius:6px;overflow:hidden;margin-top:12px}
.bar-fill{height:100%;border-radius:6px;transition:width .6s ease-out}
.bar-fill.blue{background:linear-gradient(90deg,#3b82f6,#60a5fa)}
.bar-fill.amber{background:linear-gradient(90deg,#f59e0b,#fbbf24)}
.bar-fill.red{background:linear-gradient(90deg,#ef4444,#f87171)}
.bar-fill.green{background:linear-gradient(90deg,#22c55e,#4ade80)}
.section{margin-top:4px}
.list{display:grid;gap:2px}
.row{display:flex;align-items:center;justify-content:space-between;gap:10px;border-top:1px solid rgba(255,255,255,.04);padding:12px 0}
.row:first-child{border-top:0}
.tag{display:inline-flex;align-items:center;border-radius:8px;padding:4px 10px;font-size:12px;font-weight:600;white-space:nowrap}
.tag-critical{background:rgba(239,68,68,.15);color:#f87171;border:1px solid rgba(239,68,68,.25)}
.tag-high{background:rgba(245,158,11,.12);color:#fbbf24;border:1px solid rgba(245,158,11,.2)}
.tag-medium{background:rgba(59,130,246,.12);color:#60a5fa;border:1px solid rgba(59,130,246,.2)}
.tag-low{background:rgba(34,197,94,.1);color:#4ade80;border:1px solid rgba(34,197,94,.18)}
.tag-info{background:rgba(148,163,184,.1);color:#94a3b8;border:1px solid rgba(148,163,184,.15)}
.risk-critical{color:#f87171}.risk-high{color:#fbbf24}.risk-medium{color:#60a5fa}.risk-low{color:#4ade80}
.muted{color:#64748b;font-size:13px}
.timeline-kind{font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.5px}
.tl-alert{color:#f87171;border-left:3px solid #ef4444;padding-left:12px}
.tl-action{color:#fbbf24;border-left:3px solid #f59e0b;padding-left:12px}
.tl-telemetry{color:#60a5fa;border-left:3px solid #3b82f6;padding-left:12px}
.action-steps{display:flex;flex-wrap:wrap;gap:6px;margin-top:8px}
.action-step{display:inline-flex;align-items:center;gap:4px;background:rgba(255,255,255,.04);border:1px solid rgba(255,255,255,.06);border-radius:6px;padding:3px 8px;font-size:11px;color:#94a3b8}
.action-step::before{content:"›";color:#60a5fa;font-weight:700}
footer{text-align:center;margin-top:32px;padding-top:20px;border-top:1px solid rgba(255,255,255,.04);animation:fadeIn .5s ease-out .6s backwards}
footer p{font-size:12px;color:#475569}
footer .brand{font-weight:700;background:linear-gradient(135deg,#60a5fa,#a78bfa);-webkit-background-clip:text;-webkit-text-fill-color:transparent;background-clip:text}
@media(max-width:860px){main{padding:20px 16px}header{flex-direction:column}.status-badge{margin-top:8px}.grid{grid-template-columns:repeat(2,1fr)}.wide{grid-column:span 2}}
@media(max-width:520px){.grid{grid-template-columns:1fr}.wide{grid-column:span 1}.title{font-size:24px}}"#,
    );
}

fn render_header(html: &mut String, dashboard: &DashboardState) {
    let status_class = status_dot_class(dashboard.telemetry.agent_status);
    write!(
        html,
        "<header><div><div class=\"eyebrow\">Observe-only demo</div>\
         <h1 class=\"title\">SentraEDR Dashboard</h1>\
         <p class=\"subtitle\">Synthetic telemetry projection · Alert review · Remediation queue</p></div>\
         <div class=\"status-badge\"><span class=\"dot {status_class}\"></span>{}</div></header>",
        status_label(dashboard.telemetry.agent_status)
    )
    .expect("write to string should not fail");
}

fn render_telemetry_grid(html: &mut String, dashboard: &DashboardState) {
    html.push_str("<section class=\"grid\" aria-label=\"Live Telemetry\">");

    // Row 1: Core metrics
    metric_panel_with_bar(
        html,
        "Events Received",
        dashboard.telemetry.total_received,
        "Total ETW events ingested",
        100,
        "blue",
    );
    metric_panel_with_bar(
        html,
        "Normalized",
        dashboard.telemetry.normalized_events,
        "Ready for analysis",
        percentage(
            dashboard.telemetry.normalized_events,
            dashboard.telemetry.total_received,
        ),
        "green",
    );
    metric_panel_with_bar(
        html,
        "Behavioral Signals",
        dashboard.telemetry.behavioral_signals,
        "Process · Persistence · Network · Memory",
        percentage(
            dashboard.telemetry.behavioral_signals,
            dashboard.telemetry.normalized_events,
        ),
        "blue",
    );
    metric_panel_with_bar(
        html,
        "Detection Alerts",
        dashboard.telemetry.detection_alerts,
        "Observe-only alerts raised",
        if dashboard.telemetry.detection_alerts > 0 {
            100
        } else {
            0
        },
        if dashboard.telemetry.detection_alerts > 2 {
            "red"
        } else {
            "amber"
        },
    );

    // Row 2: IPC and drop metrics
    metric_panel_with_bar(
        html,
        "Dropped Events",
        dashboard.telemetry.dropped_events,
        "Backpressure indicator",
        percentage(
            dashboard.telemetry.dropped_events,
            dashboard.telemetry.total_received,
        ),
        "red",
    );
    metric_panel(
        html,
        "IPC Frames Accepted",
        dashboard.telemetry.ipc_frames_accepted,
        "In-memory demo pipeline",
    );
    metric_panel(
        html,
        "IPC Failed Frames",
        dashboard.telemetry.ipc_failed_frames,
        "Decode or dispatch failures",
    );

    // IPC Status panel
    html.push_str("<article class=\"panel\"><h2>IPC Status</h2>");
    write!(
        html,
        "<div class=\"metric\">{}</div><div class=\"label\">Dispatcher capacity {}</div>",
        if dashboard.telemetry.ipc_enabled {
            "Enabled"
        } else {
            "Disabled"
        },
        dashboard.telemetry.ipc_dispatcher_capacity
    )
    .expect("write to string should not fail");
    html.push_str("</article></section>");
}

fn render_lower_grid(html: &mut String, dashboard: &DashboardState) {
    html.push_str("<section class=\"grid section\">");
    render_risk_summary(html, dashboard);
    render_alerts(html, dashboard);
    render_pending_actions(html, dashboard);
    render_timeline(html, dashboard);
    html.push_str("</section>");
}

fn metric_panel(html: &mut String, title: &str, value: u64, label: &str) {
    write!(
        html,
        "<article class=\"panel\"><h2>{}</h2><div class=\"metric\">{value}</div>\
         <div class=\"label\">{}</div></article>",
        escape_html(title),
        escape_html(label)
    )
    .expect("write to string should not fail");
}

fn metric_panel_with_bar(
    html: &mut String,
    title: &str,
    value: u64,
    label: &str,
    bar_percent: u64,
    bar_class: &str,
) {
    let clamped = bar_percent.min(100);
    write!(
        html,
        "<article class=\"panel\"><h2>{}</h2><div class=\"metric\">{value}</div>\
         <div class=\"label\">{}</div>\
         <div class=\"bar-track\"><div class=\"bar-fill {bar_class}\" style=\"width:{clamped}%\"></div></div>\
         </article>",
        escape_html(title),
        escape_html(label)
    )
    .expect("write to string should not fail");
}

fn render_risk_summary(html: &mut String, dashboard: &DashboardState) {
    html.push_str("<article class=\"panel wide\"><h2>Risk Summary</h2><div class=\"list\">");
    summary_row(
        html,
        "Critical Alerts",
        dashboard.summary.critical,
        "risk-critical",
    );
    summary_row(html, "High Alerts", dashboard.summary.high, "risk-high");
    summary_row(
        html,
        "Medium Alerts",
        dashboard.summary.medium,
        "risk-medium",
    );
    summary_row(html, "Low Alerts", dashboard.summary.low, "risk-low");
    summary_row(
        html,
        "Remediation Eligible",
        dashboard.summary.remediation_eligible,
        "",
    );
    html.push_str("</div></article>");
}

fn summary_row(html: &mut String, label: &str, value: usize, class_name: &str) {
    write!(
        html,
        "<div class=\"row\"><span class=\"{}\">{}</span><strong>{value}</strong></div>",
        escape_html(class_name),
        escape_html(label)
    )
    .expect("write to string should not fail");
}

fn render_alerts(html: &mut String, dashboard: &DashboardState) {
    html.push_str("<article class=\"panel wide\"><h2>Alert Review</h2><div class=\"list\">");
    if dashboard.alerts.is_empty() {
        html.push_str("<div class=\"muted\">No alerts in the current demo state.</div>");
    }
    for alert in &dashboard.alerts {
        let tag_class = risk_tag_class(alert.risk_level);
        write!(
            html,
            "<div class=\"row\"><div><strong>{} — score {}</strong>\
             <div class=\"label\">{} signals · {} MITRE techniques</div></div>\
             <span class=\"tag {tag_class}\">{}</span></div>",
            risk_label(alert.risk_level),
            alert.score,
            alert.signal_count,
            alert.mitre_technique_count,
            escape_html(&alert.recommended_action)
        )
        .expect("write to string should not fail");
    }
    html.push_str("</div></article>");
}

fn render_pending_actions(html: &mut String, dashboard: &DashboardState) {
    html.push_str("<article class=\"panel wide\"><h2>Pending Actions</h2><div class=\"list\">");
    if dashboard.pending_actions.is_empty() {
        html.push_str("<div class=\"muted\">No actions waiting for approval.</div>");
    }
    for action in &dashboard.pending_actions {
        write!(
            html,
            "<div class=\"row\"><div><strong>{}</strong>\
             <div class=\"label\">{} steps queued · {} · {}</div>\
             <div class=\"action-steps\">",
            escape_html(&action.title),
            action.actions.len(),
            escape_html(&format!("{:?}", action.mode)),
            escape_html(&action.queued_at.to_rfc3339())
        )
        .expect("write to string should not fail");
        for item in &action.actions {
            write!(
                html,
                "<span class=\"action-step\">{}</span>",
                escape_html(action_label(*item))
            )
            .expect("write to string should not fail");
        }
        html.push_str("</div></div></div>");
    }
    html.push_str("</div></article>");
}

fn render_timeline(html: &mut String, dashboard: &DashboardState) {
    html.push_str("<article class=\"panel wide\"><h2>Event Timeline</h2><div class=\"list\">");
    if dashboard.timeline.is_empty() {
        html.push_str("<div class=\"muted\">Timeline is empty.</div>");
    }
    for entry in &dashboard.timeline {
        let tl_class = timeline_class(entry.kind);
        write!(
            html,
            "<div class=\"row\"><div class=\"{tl_class}\"><span class=\"timeline-kind\">{}</span>\
             <div>{}</div></div>\
             <span class=\"muted\">{}</span></div>",
            timeline_label(entry.kind),
            escape_html(&entry.title),
            escape_html(&entry.timestamp.to_rfc3339())
        )
        .expect("write to string should not fail");
    }
    html.push_str("</div></article>");
}

fn render_footer(html: &mut String, dashboard: &DashboardState) {
    write!(
        html,
        "<footer><p><span class=\"brand\">SentraEDR</span> · Observe-only demo dashboard · \
         Generated {}</p></footer>",
        escape_html(&dashboard.generated_at.to_rfc3339())
    )
    .expect("write to string should not fail");
}

fn percentage(part: u64, total: u64) -> u64 {
    if total == 0 {
        return 0;
    }
    (part * 100) / total
}

fn status_label(status: HealthStatus) -> &'static str {
    match status {
        HealthStatus::Healthy => "Healthy",
        HealthStatus::Degraded => "Degraded",
        HealthStatus::Stopped => "Stopped",
    }
}

fn status_dot_class(status: HealthStatus) -> &'static str {
    match status {
        HealthStatus::Healthy => "healthy",
        HealthStatus::Degraded => "degraded",
        HealthStatus::Stopped => "stopped",
    }
}

fn risk_label(risk: RiskLevel) -> &'static str {
    match risk {
        RiskLevel::Informational => "Informational",
        RiskLevel::Low => "Low",
        RiskLevel::Medium => "Medium",
        RiskLevel::High => "High",
        RiskLevel::Critical => "Critical",
    }
}

fn risk_tag_class(risk: RiskLevel) -> &'static str {
    match risk {
        RiskLevel::Informational => "tag-info",
        RiskLevel::Low => "tag-low",
        RiskLevel::Medium => "tag-medium",
        RiskLevel::High => "tag-high",
        RiskLevel::Critical => "tag-critical",
    }
}

fn timeline_label(kind: TimelineKind) -> &'static str {
    match kind {
        TimelineKind::AlertObserved => "Alert",
        TimelineKind::ActionQueued => "Action",
        TimelineKind::TelemetryUpdated => "Telemetry",
    }
}

fn timeline_class(kind: TimelineKind) -> &'static str {
    match kind {
        TimelineKind::AlertObserved => "tl-alert",
        TimelineKind::ActionQueued => "tl-action",
        TimelineKind::TelemetryUpdated => "tl-telemetry",
    }
}

fn action_label(action: RemediationAction) -> &'static str {
    match action {
        RemediationAction::SuspendProcess => "Suspend process",
        RemediationAction::IsolateNetwork => "Isolate network",
        RemediationAction::QuarantineFile => "Quarantine file",
        RemediationAction::BackupRegistryValue => "Backup registry value",
        RemediationAction::RestoreRegistryValue => "Restore registry value",
    }
}

fn escape_html(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(character),
        }
    }
    escaped
}
