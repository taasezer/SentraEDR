use sentra_ui::{ActionReviewCard, DashboardState, TimelineKind};
use shared_models::{
    Alert, Finding, RemediationAction, RemediationMode, RiskLevel, Signal, Timestamp,
};

#[test]
fn dashboard_summarizes_alert_risk_counts() {
    let dashboard = DashboardState::from_alerts(vec![
        alert(RiskLevel::Critical, 95, true),
        alert(RiskLevel::High, 80, true),
        alert(RiskLevel::Low, 25, false),
    ]);

    assert_eq!(dashboard.summary.total_alerts, 3);
    assert_eq!(dashboard.summary.critical, 1);
    assert_eq!(dashboard.summary.high, 1);
    assert_eq!(dashboard.summary.low, 1);
    assert_eq!(dashboard.summary.remediation_eligible, 2);
}

#[test]
fn alert_cards_are_sorted_by_score_descending() {
    let dashboard = DashboardState::from_alerts(vec![
        alert(RiskLevel::Low, 25, false),
        alert(RiskLevel::Critical, 95, true),
        alert(RiskLevel::High, 80, true),
    ]);

    let scores: Vec<u8> = dashboard.alerts.iter().map(|alert| alert.score).collect();
    assert_eq!(scores, vec![95, 80, 25]);
}

#[test]
fn timeline_contains_alert_entries_in_timestamp_order() {
    let dashboard = DashboardState::from_alerts(vec![
        alert_at(RiskLevel::High, 80, "2026-06-27T09:12:00Z"),
        alert_at(RiskLevel::Critical, 95, "2026-06-27T09:10:00Z"),
    ]);

    assert_eq!(dashboard.timeline.len(), 2);
    assert_eq!(dashboard.timeline[0].kind, TimelineKind::AlertObserved);
    assert_eq!(
        dashboard.timeline[0].timestamp.to_rfc3339(),
        "2026-06-27T09:10:00+00:00"
    );
}

#[test]
fn pending_action_cards_are_added_to_dashboard() {
    let mut dashboard = DashboardState::from_alerts(vec![alert(RiskLevel::High, 80, true)]);
    dashboard.add_pending_action(ActionReviewCard::new(
        "approval-required remediation",
        RemediationMode::ApprovalRequired,
        vec![
            RemediationAction::SuspendProcess,
            RemediationAction::QuarantineFile,
        ],
        ts("2026-06-27T09:13:00Z"),
    ));

    assert_eq!(dashboard.pending_actions.len(), 1);
    assert_eq!(dashboard.summary.pending_actions, 1);
    assert_eq!(dashboard.timeline.last().unwrap().kind, TimelineKind::ActionQueued);
}

fn alert(risk_level: RiskLevel, score: u8, remediation_eligible: bool) -> Alert {
    alert_at_eligible(
        risk_level,
        score,
        remediation_eligible,
        "2026-06-27T09:11:00Z",
    )
}

fn alert_at(risk_level: RiskLevel, score: u8, timestamp: &str) -> Alert {
    alert_at_eligible(risk_level, score, true, timestamp)
}

fn alert_at_eligible(
    risk_level: RiskLevel,
    score: u8,
    remediation_eligible: bool,
    timestamp: &str,
) -> Alert {
    let mut finding = Finding::new(ts(timestamp), risk_level, score);
    finding.signals.push(Signal {
        name: "synthetic_signal".to_string(),
        description: "synthetic signal".to_string(),
        supporting_event_ids: Vec::new(),
    });
    finding.mitre_techniques.push("T1059.001".to_string());

    Alert {
        alert_id: Default::default(),
        finding,
        recommended_action: "review alert".to_string(),
        remediation_eligible,
    }
}

fn ts(value: &str) -> Timestamp {
    Timestamp::parse_rfc3339(value).unwrap()
}
