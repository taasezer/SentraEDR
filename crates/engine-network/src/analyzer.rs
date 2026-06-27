use crate::event::NetworkEvent;
use crate::signal::{NetworkSignal, signals_for_event};
use shared_models::{ComponentHealth, HealthStatus, NormalizedTelemetryEvent, Timestamp};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NetworkAnalysisStats {
    pub observed: u64,
    pub handled: u64,
    pub ignored: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkAnalysisReport {
    pub stats: NetworkAnalysisStats,
    pub tracked_destinations: usize,
    pub signals: Vec<NetworkSignal>,
    pub component_health: ComponentHealth,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestinationHistory {
    pub destination: String,
    pub remote_ip: String,
    pub remote_port: u16,
    pub first_observed: Timestamp,
    pub last_observed: Timestamp,
    pub observation_count: u64,
    pub last_interval_seconds: Option<i64>,
    pub previous_interval_seconds: Option<i64>,
}

#[derive(Debug, Default)]
pub struct NetworkAnalyzer {
    stats: NetworkAnalysisStats,
    history: BTreeMap<String, DestinationHistory>,
}

impl NetworkAnalyzer {
    pub fn analyze(&mut self, event: NormalizedTelemetryEvent) -> NetworkAnalysisReport {
        self.stats.observed += 1;
        let mut signals = Vec::new();

        match NetworkEvent::from_telemetry(&event) {
            Some(network_event) => {
                self.stats.handled += 1;
                let key = network_event.destination_key();
                let history = self.update_history(key, &network_event);
                signals = signals_for_event(&network_event, history);
            }
            None => self.stats.ignored += 1,
        }

        NetworkAnalysisReport {
            stats: self.stats.clone(),
            tracked_destinations: self.history.len(),
            signals,
            component_health: ComponentHealth {
                component: "engine-network".to_string(),
                status: HealthStatus::Healthy,
                observed_at: Timestamp::now(),
                queue: None,
            },
        }
    }

    fn update_history(&mut self, key: String, event: &NetworkEvent) -> &DestinationHistory {
        let current_seconds = timestamp_seconds(&event.observed_at);
        let entry = self
            .history
            .entry(key.clone())
            .or_insert(DestinationHistory {
                destination: key,
                remote_ip: event.remote_ip.clone(),
                remote_port: event.remote_port,
                first_observed: event.observed_at.clone(),
                last_observed: event.observed_at.clone(),
                observation_count: 0,
                last_interval_seconds: None,
                previous_interval_seconds: None,
            });

        if entry.observation_count > 0 {
            let previous_seconds = timestamp_seconds(&entry.last_observed);
            entry.previous_interval_seconds = entry.last_interval_seconds;
            entry.last_interval_seconds = Some(current_seconds - previous_seconds);
        }
        entry.last_observed = event.observed_at.clone();
        entry.observation_count += 1;
        entry
    }
}

fn timestamp_seconds(timestamp: &Timestamp) -> i64 {
    let value = timestamp.to_rfc3339();
    let hour = value
        .get(11..13)
        .and_then(|v| v.parse::<i64>().ok())
        .unwrap_or(0);
    let minute = value
        .get(14..16)
        .and_then(|v| v.parse::<i64>().ok())
        .unwrap_or(0);
    let second = value
        .get(17..19)
        .and_then(|v| v.parse::<i64>().ok())
        .unwrap_or(0);
    hour * 3600 + minute * 60 + second
}
