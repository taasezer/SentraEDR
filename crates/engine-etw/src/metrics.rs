use shared_models::{ComponentHealth, HealthStatus, QueueHealth, Timestamp};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EtwIngestionStats {
    pub received: u64,
    pub normalized: u64,
    pub dropped: u64,
    pub failed: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EtwIngestionReport {
    pub stats: EtwIngestionStats,
    pub component_health: ComponentHealth,
}

impl EtwIngestionReport {
    pub fn new(stats: EtwIngestionStats, queue: QueueHealth) -> Self {
        let status = if stats.failed > 0 || stats.dropped > 0 || queue.dropped_events > 0 {
            HealthStatus::Degraded
        } else {
            HealthStatus::Healthy
        };

        Self {
            stats,
            component_health: ComponentHealth {
                component: "engine-etw".to_string(),
                status,
                observed_at: Timestamp::now(),
                queue: Some(queue),
            },
        }
    }
}
