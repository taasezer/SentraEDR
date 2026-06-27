use crate::event::PersistenceEvent;
use crate::signal::{PersistenceSignal, signal_for_event};
use shared_models::{ComponentHealth, HealthStatus, NormalizedTelemetryEvent, Timestamp};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PersistenceAnalysisStats {
    pub observed: u64,
    pub handled: u64,
    pub ignored: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistenceAnalysisReport {
    pub stats: PersistenceAnalysisStats,
    pub signals: Vec<PersistenceSignal>,
    pub component_health: ComponentHealth,
}

#[derive(Debug, Default)]
pub struct PersistenceAnalyzer {
    stats: PersistenceAnalysisStats,
}

impl PersistenceAnalyzer {
    pub fn analyze(&mut self, event: NormalizedTelemetryEvent) -> PersistenceAnalysisReport {
        self.stats.observed += 1;
        let mut signals = Vec::new();

        match PersistenceEvent::from_telemetry(&event) {
            Some(persistence_event) => {
                self.stats.handled += 1;
                if let Some(signal) = signal_for_event(&persistence_event) {
                    signals.push(signal);
                }
            }
            None => {
                self.stats.ignored += 1;
            }
        }

        PersistenceAnalysisReport {
            stats: self.stats.clone(),
            signals,
            component_health: ComponentHealth {
                component: "engine-persistence".to_string(),
                status: HealthStatus::Healthy,
                observed_at: Timestamp::now(),
                queue: None,
            },
        }
    }
}
