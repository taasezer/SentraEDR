use crate::signal::{ProcessSignal, signals_for_start};
use crate::state::{ProcessStateTable, ProcessStateUpdate};
use shared_models::{ComponentHealth, HealthStatus, NormalizedTelemetryEvent, Timestamp};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProcessAnalysisStats {
    pub observed: u64,
    pub started: u64,
    pub exited: u64,
    pub ignored: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessAnalysisReport {
    pub stats: ProcessAnalysisStats,
    pub tracked_processes: usize,
    pub signals: Vec<ProcessSignal>,
    pub component_health: ComponentHealth,
}

#[derive(Debug, Default)]
pub struct ProcessAnalyzer {
    state: ProcessStateTable,
    stats: ProcessAnalysisStats,
}

impl ProcessAnalyzer {
    pub fn analyze(&mut self, event: NormalizedTelemetryEvent) -> ProcessAnalysisReport {
        self.stats.observed += 1;
        let mut signals = Vec::new();

        match self.state.apply_event(&event) {
            ProcessStateUpdate::Started(snapshot) => {
                self.stats.started += 1;
                let parent = snapshot
                    .process
                    .parent_process_id
                    .and_then(|parent_id| self.state.get(parent_id));
                signals = signals_for_start(&snapshot, parent, event.event_id);
            }
            ProcessStateUpdate::Exited(_) => {
                self.stats.exited += 1;
            }
            ProcessStateUpdate::Ignored => {
                self.stats.ignored += 1;
            }
        }

        ProcessAnalysisReport {
            stats: self.stats.clone(),
            tracked_processes: self.state.len(),
            signals,
            component_health: ComponentHealth {
                component: "engine-process".to_string(),
                status: HealthStatus::Healthy,
                observed_at: Timestamp::now(),
                queue: None,
            },
        }
    }
}
