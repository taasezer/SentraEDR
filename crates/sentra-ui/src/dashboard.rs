use crate::action_queue::ActionReviewCard;
use crate::alert_card::AlertCard;
use crate::timeline::{TimelineEntry, TimelineKind};
use shared_models::{Alert, RiskLevel};
use std::cmp::Reverse;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RiskSummary {
    pub total_alerts: usize,
    pub informational: usize,
    pub low: usize,
    pub medium: usize,
    pub high: usize,
    pub critical: usize,
    pub remediation_eligible: usize,
    pub pending_actions: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DashboardState {
    pub summary: RiskSummary,
    pub alerts: Vec<AlertCard>,
    pub timeline: Vec<TimelineEntry>,
    pub pending_actions: Vec<ActionReviewCard>,
}

impl DashboardState {
    pub fn from_alerts(alerts: Vec<Alert>) -> Self {
        let mut alert_cards: Vec<AlertCard> =
            alerts.into_iter().map(AlertCard::from_alert).collect();
        alert_cards.sort_by_key(|alert| Reverse(alert.score));

        let mut dashboard = Self {
            summary: summarize_alerts(&alert_cards),
            timeline: alert_cards.iter().map(alert_timeline_entry).collect(),
            alerts: alert_cards,
            pending_actions: Vec::new(),
        };
        dashboard.sort_timeline();
        dashboard
    }

    pub fn add_pending_action(&mut self, action: ActionReviewCard) {
        self.timeline.push(TimelineEntry::new(
            action.queued_at.clone(),
            TimelineKind::ActionQueued,
            action.title.clone(),
        ));
        self.pending_actions.push(action);
        self.summary.pending_actions = self.pending_actions.len();
        self.sort_timeline();
    }

    fn sort_timeline(&mut self) {
        self.timeline.sort_by(|left, right| {
            left.timestamp
                .to_rfc3339()
                .cmp(&right.timestamp.to_rfc3339())
        });
    }
}

fn summarize_alerts(alerts: &[AlertCard]) -> RiskSummary {
    let mut summary = RiskSummary {
        total_alerts: alerts.len(),
        ..Default::default()
    };

    for alert in alerts {
        match alert.risk_level {
            RiskLevel::Informational => summary.informational += 1,
            RiskLevel::Low => summary.low += 1,
            RiskLevel::Medium => summary.medium += 1,
            RiskLevel::High => summary.high += 1,
            RiskLevel::Critical => summary.critical += 1,
        }
        if alert.remediation_eligible {
            summary.remediation_eligible += 1;
        }
    }

    summary
}

fn alert_timeline_entry(alert: &AlertCard) -> TimelineEntry {
    TimelineEntry::new(
        alert.timestamp.clone(),
        TimelineKind::AlertObserved,
        format!("Alert score {}", alert.score),
    )
}
