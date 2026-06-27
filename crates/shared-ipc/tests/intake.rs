use shared_ipc::{
    IpcDispatcherConfig, IpcEnvelope, IpcFrameIntake, IpcMessageKind, IpcPayload, encode_frame,
};
use shared_models::{
    Alert, Finding, RemediationAction, RemediationCommand, RemediationMode, RiskLevel, Timestamp,
};

fn sample_timestamp() -> Timestamp {
    Timestamp::parse_rfc3339("2026-06-27T09:00:00Z").unwrap()
}

fn sample_alert() -> Alert {
    let finding = Finding::new(sample_timestamp(), RiskLevel::High, 85);
    Alert::observe_only(finding, "Review suspicious behavior")
}

fn sample_alert_envelope() -> IpcEnvelope {
    IpcEnvelope::new(
        IpcMessageKind::Alert,
        sample_timestamp(),
        IpcPayload::Alert(sample_alert()),
    )
    .unwrap()
}

fn sample_remediation_request() -> IpcEnvelope {
    let alert = sample_alert();
    let command = RemediationCommand::new(
        alert.alert_id,
        sample_timestamp(),
        "operator",
        RemediationMode::ApprovalRequired,
        RemediationAction::SuspendProcess,
        "approved in review queue",
    );

    IpcEnvelope::new(
        IpcMessageKind::RemediationRequest,
        sample_timestamp(),
        IpcPayload::RemediationRequest(command),
    )
    .unwrap()
}

#[test]
fn encoded_alert_frame_routes_to_alert_queue() {
    let mut intake = IpcFrameIntake::new(IpcDispatcherConfig::try_new(4).unwrap());
    let envelope = sample_alert_envelope();
    let frame = encode_frame(&envelope).unwrap();

    intake.accept_frame(&frame).unwrap();

    assert_eq!(intake.dispatcher_mut().alerts.try_recv(), Some(envelope));
    assert_eq!(intake.stats().accepted, 1);
    assert_eq!(intake.stats().decode_failed, 0);
    assert_eq!(intake.stats().dispatch_failed, 0);
}

#[test]
fn malformed_frame_increments_decode_failure_count() {
    let mut intake = IpcFrameIntake::new(IpcDispatcherConfig::try_new(4).unwrap());

    let result = intake.accept_frame(&[0, 0, 0, 10, b'{']);

    assert!(result.is_err());
    assert_eq!(intake.stats().accepted, 0);
    assert_eq!(intake.stats().decode_failed, 1);
    assert_eq!(intake.stats().dispatch_failed, 0);
}

#[test]
fn full_dispatch_queue_increments_dispatch_failure_count() {
    let mut intake = IpcFrameIntake::new(IpcDispatcherConfig::try_new(1).unwrap());
    let first = encode_frame(&sample_alert_envelope()).unwrap();
    let second = encode_frame(&sample_alert_envelope()).unwrap();

    intake.accept_frame(&first).unwrap();
    let result = intake.accept_frame(&second);

    assert!(result.is_err());
    assert_eq!(intake.stats().accepted, 1);
    assert_eq!(intake.stats().decode_failed, 0);
    assert_eq!(intake.stats().dispatch_failed, 1);
}

#[test]
fn remediation_request_frame_is_queued_as_data() {
    let mut intake = IpcFrameIntake::new(IpcDispatcherConfig::try_new(4).unwrap());
    let envelope = sample_remediation_request();
    let frame = encode_frame(&envelope).unwrap();

    intake.accept_frame(&frame).unwrap();

    assert_eq!(
        intake.dispatcher_mut().remediation_requests.try_recv(),
        Some(envelope)
    );
    assert_eq!(intake.stats().accepted, 1);
}
