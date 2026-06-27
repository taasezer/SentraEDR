use sentra_agent::config::IpcConfig;
use sentra_agent::ipc::IpcService;
use shared_ipc::{IpcEnvelope, IpcError, IpcMessageKind, IpcPayload, encode_frame};
use shared_models::{ComponentHealth, HealthStatus, Timestamp};

fn ipc_config(enabled: bool, dispatcher_capacity: usize) -> IpcConfig {
    IpcConfig {
        enabled,
        dispatcher_capacity,
    }
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
    .unwrap()
}

#[test]
fn service_routes_fragmented_frame_to_dispatcher() {
    let mut service = IpcService::new(ipc_config(true, 4)).unwrap();
    let frame = encode_frame(&health_envelope()).unwrap();
    let split_at = frame.len() / 2;

    service.process_raw_bytes(&frame[..split_at]).unwrap();
    service.process_raw_bytes(&frame[split_at..]).unwrap();

    let stats = service.stats();
    assert_eq!(stats.chunks_received, 2);
    assert_eq!(stats.frames_completed, 1);
    assert_eq!(stats.frames_accepted, 1);
    assert!(service.dispatcher_mut().health.try_recv().is_some());
}

#[test]
fn disabled_service_ignores_raw_bytes_without_dispatching() {
    let mut service = IpcService::new(ipc_config(false, 4)).unwrap();
    let frame = encode_frame(&health_envelope()).unwrap();

    service.process_raw_bytes(&frame).unwrap();

    let stats = service.stats();
    assert_eq!(stats.chunks_received, 0);
    assert_eq!(stats.frames_completed, 0);
    assert!(service.dispatcher_mut().health.try_recv().is_none());
}

#[test]
fn service_rejects_zero_dispatcher_capacity() {
    let result = IpcService::new(ipc_config(true, 0));

    assert!(matches!(
        result,
        Err(IpcError::InvalidDispatcherCapacity { capacity: 0 })
    ));
}
