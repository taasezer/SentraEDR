use shared_ipc::{
    IpcDispatcherConfig, IpcEnvelope, IpcMessageKind, IpcPayload, IpcPipeline, encode_frame,
};
use shared_models::{ComponentHealth, HealthStatus, Timestamp};

fn create_test_frame(kind: IpcMessageKind, payload: IpcPayload) -> Vec<u8> {
    let envelope =
        IpcEnvelope::new(kind, Timestamp::now(), payload).expect("Envelope creation failed");
    encode_frame(&envelope).expect("Encoding failed")
}

fn dummy_health() -> ComponentHealth {
    ComponentHealth {
        component: "test".to_string(),
        status: HealthStatus::Healthy,
        observed_at: Timestamp::now(),
        queue: None,
    }
}

#[test]
fn test_pipeline_happy_path() {
    let config = IpcDispatcherConfig { queue_capacity: 10 };
    let mut pipeline = IpcPipeline::new(config);

    let frame = create_test_frame(IpcMessageKind::Health, IpcPayload::Health(dummy_health()));

    // Split frame into two chunks
    let mid = frame.len() / 2;
    pipeline
        .process_bytes(&frame[..mid])
        .expect("First chunk failed");
    pipeline
        .process_bytes(&frame[mid..])
        .expect("Second chunk failed");

    let stats = pipeline.stats();
    assert_eq!(stats.chunks_received, 2);
    assert_eq!(stats.frames_completed, 1);
    assert_eq!(stats.frames_accepted, 1);

    // Verify the message reached the dispatcher
    let msg = pipeline.dispatcher_mut().health.try_recv();
    assert!(msg.is_some());
}

#[test]
fn test_pipeline_fragmented_frames() {
    let config = IpcDispatcherConfig { queue_capacity: 10 };
    let mut pipeline = IpcPipeline::new(config);

    let frame1 = create_test_frame(IpcMessageKind::Health, IpcPayload::Health(dummy_health()));
    let frame2 = create_test_frame(IpcMessageKind::Health, IpcPayload::Health(dummy_health()));

    let mut all_bytes = Vec::new();
    all_bytes.extend(frame1);
    all_bytes.extend(frame2);

    // Send in 3 odd chunks
    let chunks = [
        &all_bytes[..2],
        &all_bytes[2..all_bytes.len() - 2],
        &all_bytes[all_bytes.len() - 2..],
    ];

    for chunk in chunks {
        pipeline.process_bytes(chunk).expect("Chunk failed");
    }

    let stats = pipeline.stats();
    assert_eq!(stats.chunks_received, 3);
    assert_eq!(stats.frames_completed, 2);
    assert_eq!(stats.frames_accepted, 2);

    let health_rx = pipeline.dispatcher_mut();
    assert!(health_rx.health.try_recv().is_some());
    assert!(health_rx.health.try_recv().is_some());
}

#[test]
fn test_pipeline_malformed_frame() {
    let config = IpcDispatcherConfig { queue_capacity: 10 };
    let mut pipeline = IpcPipeline::new(config);

    // Create a frame with a valid length but invalid payload
    let mut malformed = vec![0, 0, 0, 10]; // Length 10
    malformed.extend(vec![0u8; 10]);

    pipeline
        .process_bytes(&malformed)
        .expect("Chunk should be accepted");

    let stats = pipeline.stats();
    assert_eq!(stats.frames_completed, 1);
    assert_eq!(stats.intake_decode_failed, 1);
    assert_eq!(stats.frames_accepted, 0);
}

#[test]
fn test_pipeline_buffer_overflow() {
    let config = IpcDispatcherConfig { queue_capacity: 10 };
    let mut pipeline = IpcPipeline::new(config);

    // Send a chunk larger than MAX_BUFFERED_BYTES (roughly 1MB)
    let huge_chunk = vec![0u8; 2 * 1024 * 1024];
    let result = pipeline.process_bytes(&huge_chunk);

    assert!(result.is_err());
    assert_eq!(pipeline.stats().stream_rejected, 1);
}

#[test]
fn test_pipeline_dispatch_failure() {
    let config = IpcDispatcherConfig { queue_capacity: 1 };
    let mut pipeline = IpcPipeline::new(config);

    let frame = create_test_frame(IpcMessageKind::Health, IpcPayload::Health(dummy_health()));

    // First frame: accepted
    pipeline.process_bytes(&frame).expect("First failed");

    // Second frame: queue full (capacity 1)
    pipeline
        .process_bytes(&frame)
        .expect("Second should be processed by pipeline but fail in dispatch");

    let stats = pipeline.stats();
    assert_eq!(stats.frames_completed, 2);
    assert_eq!(stats.intake_dispatch_failed, 1);
    assert_eq!(stats.frames_accepted, 1);
}
