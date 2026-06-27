use crate::config::AgentConfig;
use crate::ipc::IpcService;
use shared_ipc::{
    IpcEnvelope, IpcError, IpcMessageKind, IpcPayload, IpcPipelineStats, encode_frame,
};
use shared_models::{ComponentHealth, HealthStatus, Timestamp};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntheticIpcDryRunReport {
    pub stats: IpcPipelineStats,
    pub delivered_health_messages: usize,
}

pub fn run_synthetic_ipc_dry_run() -> Result<SyntheticIpcDryRunReport, IpcError> {
    let config = AgentConfig::default();
    let mut service = IpcService::new(config.ipc)?;
    let frame = encode_frame(&health_envelope())?;
    let split_at = frame.len() / 2;

    service.process_raw_bytes(&frame[..split_at])?;
    service.process_raw_bytes(&frame[split_at..])?;

    let mut delivered_health_messages = 0;
    while service.dispatcher_mut().health.try_recv().is_some() {
        delivered_health_messages += 1;
    }

    Ok(SyntheticIpcDryRunReport {
        stats: service.stats(),
        delivered_health_messages,
    })
}

fn health_envelope() -> IpcEnvelope {
    let health = ComponentHealth {
        component: "sentra-agent".to_string(),
        status: HealthStatus::Healthy,
        observed_at: Timestamp::now(),
        queue: None,
    };

    IpcEnvelope::new(
        IpcMessageKind::Health,
        Timestamp::now(),
        IpcPayload::Health(health),
    )
    .expect("synthetic IPC health envelope should be valid")
}
