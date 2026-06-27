use shared_models::{NormalizedTelemetryEvent, ProcessIdentity, TelemetryAction, Timestamp};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessLifecycleStatus {
    Running,
    Exited,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessSnapshot {
    pub process: ProcessIdentity,
    pub first_observed: Timestamp,
    pub last_observed: Timestamp,
    pub status: ProcessLifecycleStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessStateUpdate {
    Started(ProcessSnapshot),
    Exited(ProcessSnapshot),
    Ignored,
}

#[derive(Debug, Default)]
pub struct ProcessStateTable {
    processes: BTreeMap<u32, ProcessSnapshot>,
}

impl ProcessStateTable {
    pub fn apply_event(&mut self, event: &NormalizedTelemetryEvent) -> ProcessStateUpdate {
        let Some(process) = event.process.clone() else {
            return ProcessStateUpdate::Ignored;
        };

        match event.action {
            TelemetryAction::ProcessStarted => self.apply_start(process, event.timestamp.clone()),
            TelemetryAction::ProcessExited => self.apply_exit(process, event.timestamp.clone()),
            _ => ProcessStateUpdate::Ignored,
        }
    }

    pub fn get(&self, process_id: u32) -> Option<&ProcessSnapshot> {
        self.processes.get(&process_id)
    }

    pub fn len(&self) -> usize {
        self.processes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.processes.is_empty()
    }

    fn apply_start(
        &mut self,
        process: ProcessIdentity,
        observed_at: Timestamp,
    ) -> ProcessStateUpdate {
        let snapshot = ProcessSnapshot {
            process,
            first_observed: observed_at.clone(),
            last_observed: observed_at,
            status: ProcessLifecycleStatus::Running,
        };
        self.processes
            .insert(snapshot.process.process_id, snapshot.clone());
        ProcessStateUpdate::Started(snapshot)
    }

    fn apply_exit(
        &mut self,
        process: ProcessIdentity,
        observed_at: Timestamp,
    ) -> ProcessStateUpdate {
        let process_id = process.process_id;
        let snapshot = match self.processes.get_mut(&process_id) {
            Some(existing) => {
                existing.last_observed = observed_at;
                existing.status = ProcessLifecycleStatus::Exited;
                existing.clone()
            }
            None => {
                let snapshot = ProcessSnapshot {
                    process,
                    first_observed: observed_at.clone(),
                    last_observed: observed_at,
                    status: ProcessLifecycleStatus::Exited,
                };
                self.processes.insert(process_id, snapshot.clone());
                snapshot
            }
        };

        ProcessStateUpdate::Exited(snapshot)
    }
}
