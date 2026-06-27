use crate::event::MemoryEvent;
use crate::signal::{MemorySignal, signal_for_event};
use shared_models::{ComponentHealth, HealthStatus, NormalizedTelemetryEvent, Timestamp};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MemoryAnalysisStats {
    pub observed: u64,
    pub handled: u64,
    pub ignored: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryAnalysisReport {
    pub stats: MemoryAnalysisStats,
    pub signals: Vec<MemorySignal>,
    pub component_health: ComponentHealth,
}

#[derive(Debug, Default)]
pub struct MemoryAnalyzer {
    stats: MemoryAnalysisStats,
}

impl MemoryAnalyzer {
    pub fn analyze(&mut self, event: NormalizedTelemetryEvent) -> MemoryAnalysisReport {
        self.stats.observed += 1;
        let mut signals = Vec::new();

        match MemoryEvent::from_telemetry(&event) {
            Some(memory_event) => {
                self.stats.handled += 1;
                if let Some(signal) = signal_for_event(&memory_event) {
                    signals.push(signal);
                }
            }
            None => self.stats.ignored += 1,
        }

        MemoryAnalysisReport {
            stats: self.stats.clone(),
            signals,
            component_health: ComponentHealth {
                component: "engine-memory".to_string(),
                status: HealthStatus::Healthy,
                observed_at: Timestamp::now(),
                queue: None,
            },
        }
    }
}
