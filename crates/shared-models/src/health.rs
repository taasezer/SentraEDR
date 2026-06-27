use crate::time::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Stopped,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueHealth {
    pub name: String,
    pub capacity: usize,
    pub depth: usize,
    pub dropped_events: u64,
}

impl QueueHealth {
    pub fn new(name: impl Into<String>, capacity: usize, depth: usize, dropped_events: u64) -> Self {
        Self {
            name: name.into(),
            capacity,
            depth,
            dropped_events,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub component: String,
    pub status: HealthStatus,
    pub observed_at: Timestamp,
    pub queue: Option<QueueHealth>,
}
