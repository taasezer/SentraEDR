use shared_ipc::{IpcError, bounded_channel};

#[tokio::test]
async fn queue_tracks_depth_after_send_and_receive() {
    let (sender, mut receiver) = bounded_channel("telemetry", 2);

    sender.try_send("first").unwrap();
    assert_eq!(sender.snapshot().depth, 1);

    let received = receiver.try_recv();
    assert_eq!(received, Some("first"));
    assert_eq!(receiver.snapshot().depth, 0);
}

#[tokio::test]
async fn queue_reports_full_without_unbounded_growth() {
    let (sender, _receiver) = bounded_channel("telemetry", 1);

    sender.try_send("first").unwrap();
    let error = sender.try_send("second").unwrap_err();

    assert_eq!(
        error,
        IpcError::QueueFull {
            queue: "telemetry".to_string(),
            capacity: 1,
        }
    );
    assert_eq!(sender.snapshot().depth, 1);
    assert_eq!(sender.snapshot().dropped_events, 1);
}
