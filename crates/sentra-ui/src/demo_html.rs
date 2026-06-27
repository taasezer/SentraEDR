use crate::{DashboardState, TimelineKind};
use shared_models::{HealthStatus, RemediationAction, RiskLevel};
use std::fmt::Write;

pub fn render_dashboard_html(dashboard: &DashboardState) -> String {
    let mut html = String::new();
    html.push_str("<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\">");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">");
    html.push_str("<title>SentraEDR Demo Dashboard</title>");
    html.push_str("<style>");
    html.push_str(
        "*,*::before,*::after{box-sizing:border-box}body{margin:0;background:#f4f7f9;color:#17212b;font-family:Segoe UI,Arial,sans-serif;letter-spacing:0}main{max-width:1180px;margin:0 auto;padding:24px}header{display:flex;align-items:flex-start;justify-content:space-between;gap:16px;margin-bottom:18px}.eyebrow{font-size:12px;font-weight:700;text-transform:uppercase;color:#476173}.title{font-size:28px;line-height:1.15;margin:4px 0}.subtitle{margin:0;color:#526779;font-size:14px}.status{display:inline-flex;align-items:center;gap:8px;border:1px solid #c8d6df;border-radius:8px;padding:8px 10px;background:#fff;font-weight:700}.dot{width:10px;height:10px;border-radius:50%;background:#16a34a}.dot.degraded{background:#d97706}.dot.stopped{background:#64748b}.grid{display:grid;grid-template-columns:repeat(4,minmax(0,1fr));gap:12px}.panel{background:#fff;border:1px solid #d7e1e7;border-radius:8px;padding:14px}.panel h2{font-size:15px;margin:0 0 12px}.metric{font-size:26px;font-weight:800;line-height:1}.label{color:#526779;font-size:12px;margin-top:5px}.wide{grid-column:span 2}.section{margin-top:14px}.list{display:grid;gap:8px}.row{display:flex;align-items:center;justify-content:space-between;gap:10px;border-top:1px solid #edf2f5;padding:9px 0}.row:first-child{border-top:0}.tag{display:inline-flex;align-items:center;border:1px solid #c8d6df;border-radius:8px;padding:3px 7px;font-size:12px;font-weight:700;background:#f8fafc}.risk-critical{color:#b91c1c}.risk-high{color:#b45309}.risk-medium{color:#0369a1}.risk-low{color:#15803d}.muted{color:#526779}.timeline-kind{font-size:12px;color:#476173;font-weight:700}.bar{height:8px;background:#e2e8f0;border-radius:8px;overflow:hidden;margin-top:10px}.bar span{display:block;height:100%;background:#2563eb}@media(max-width:760px){main{padding:16px}header{display:block}.status{margin-top:12px}.grid{grid-template-columns:1fr}.wide{grid-column:span 1}}",
    );
    html.push_str("</style></head><body><main>");

    let status_class = status_class(dashboard.telemetry.agent_status);
    write!(
        html,
        "<header><div><div class=\"eyebrow\">Observe-only demo</div><h1 class=\"title\">SentraEDR Demo Dashboard</h1><p class=\"subtitle\">Live telemetry projection, alert review, and remediation queue preview.</p></div><div class=\"status\"><span class=\"dot {status_class}\"></span>{}</div></header>",
        status_label(dashboard.telemetry.agent_status)
    )
    .expect("write to string should not fail");

    html.push_str("<section class=\"grid\" aria-label=\"Live Telemetry\">");
    metric_panel(
        &mut html,
        "Live Telemetry",
        dashboard.telemetry.total_received,
        "Events received",
    );
    metric_panel(
        &mut html,
        "Normalized Events",
        dashboard.telemetry.normalized_events,
        "Ready for analysis",
    );
    metric_panel(
        &mut html,
        "Behavioral Signals",
        dashboard.telemetry.behavioral_signals,
        "Process, persistence, network, memory",
    );
    metric_panel(
        &mut html,
        "Detection Alerts",
        dashboard.telemetry.detection_alerts,
        "Observe-only alerts",
    );
    metric_panel(
        &mut html,
        "Dropped Events",
        dashboard.telemetry.dropped_events,
        "Backpressure indicator",
    );
    metric_panel(
        &mut html,
        "IPC Frames Accepted",
        dashboard.telemetry.ipc_frames_accepted,
        "In-memory demo pipeline",
    );
    metric_panel(
        &mut html,
        "IPC Failed Frames",
        dashboard.telemetry.ipc_failed_frames,
        "Decode or dispatch failures",
    );
    html.push_str("<article class=\"panel\"><h2>IPC Status</h2>");
    write!(
        html,
        "<div class=\"metric\">{}</div><div class=\"label\">Capacity {}</div>",
        if dashboard.telemetry.ipc_enabled {
            "Enabled"
        } else {
            "Disabled"
        },
        dashboard.telemetry.ipc_dispatcher_capacity
    )
    .expect("write to string should not fail");
    html.push_str("</article></section>");

    html.push_str("<section class=\"grid section\">");
    render_risk_summary(&mut html, dashboard);
    render_alerts(&mut html, dashboard);
    render_pending_actions(&mut html, dashboard);
    render_timeline(&mut html, dashboard);
    html.push_str("</section>");

    html.push_str("</main></body></html>");
    html
}

fn metric_panel(html: &mut String, title: &str, value: u64, label: &str) {
    write!(
        html,
        "<article class=\"panel\"><h2>{}</h2><div class=\"metric\">{value}</div><div class=\"label\">{}</div></article>",
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
        write!(
            html,
            "<div class=\"row\"><div><strong class=\"{}\">{} score {}</strong><div class=\"label\">{} signals, {} MITRE techniques</div></div><span class=\"tag\">{}</span></div>",
            risk_class(alert.risk_level),
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
            "<div class=\"row\"><div><strong>{}</strong><div class=\"label\">{} actions queued at {}</div></div><span class=\"tag\">{}</span></div>",
            escape_html(&action.title),
            action.actions.len(),
            escape_html(&action.queued_at.to_rfc3339()),
            escape_html(&format!("{:?}", action.mode))
        )
        .expect("write to string should not fail");
        html.push_str("<div class=\"label\">");
        for (index, item) in action.actions.iter().enumerate() {
            if index > 0 {
                html.push_str(", ");
            }
            html.push_str(&escape_html(action_label(*item)));
        }
        html.push_str("</div>");
    }
    html.push_str("</div></article>");
}

fn render_timeline(html: &mut String, dashboard: &DashboardState) {
    html.push_str("<article class=\"panel wide\"><h2>Event Timeline</h2><div class=\"list\">");
    if dashboard.timeline.is_empty() {
        html.push_str("<div class=\"muted\">Timeline is empty.</div>");
    }
    for entry in &dashboard.timeline {
        write!(
            html,
            "<div class=\"row\"><div><span class=\"timeline-kind\">{}</span><div>{}</div></div><span class=\"muted\">{}</span></div>",
            timeline_label(entry.kind),
            escape_html(&entry.title),
            escape_html(&entry.timestamp.to_rfc3339())
        )
        .expect("write to string should not fail");
    }
    html.push_str("</div></article>");
}

fn status_label(status: HealthStatus) -> &'static str {
    match status {
        HealthStatus::Healthy => "Healthy",
        HealthStatus::Degraded => "Degraded",
        HealthStatus::Stopped => "Stopped",
    }
}

fn status_class(status: HealthStatus) -> &'static str {
    match status {
        HealthStatus::Healthy => "",
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

fn risk_class(risk: RiskLevel) -> &'static str {
    match risk {
        RiskLevel::Informational | RiskLevel::Low => "risk-low",
        RiskLevel::Medium => "risk-medium",
        RiskLevel::High => "risk-high",
        RiskLevel::Critical => "risk-critical",
    }
}

fn timeline_label(kind: TimelineKind) -> &'static str {
    match kind {
        TimelineKind::AlertObserved => "Alert",
        TimelineKind::ActionQueued => "Action",
        TimelineKind::TelemetryUpdated => "Telemetry",
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
