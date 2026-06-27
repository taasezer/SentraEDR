use shared_ipc::{
    IpcDispatcher, IpcDispatcherConfig, IpcEnvelope, IpcError, IpcMessageKind, IpcPayload,
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
fn alert_envelopes_route_to_alert_queue() {
    let mut dispatcher = IpcDispatcher::new(IpcDispatcherConfig::try_new(4).unwrap());
    let envelope = sample_alert_envelope();

    dispatcher.dispatch(envelope.clone()).unwrap();

    assert_eq!(dispatcher.alerts.try_recv(), Some(envelope));
    assert_eq!(dispatcher.alert_stats().accepted, 1);
}

#[test]
fn remediation_requests_route_to_remediation_request_queue() {
    let mut dispatcher = IpcDispatcher::new(IpcDispatcherConfig::try_new(4).unwrap());
    let envelope = sample_remediation_request();

    dispatcher.dispatch(envelope.clone()).unwrap();

    assert_eq!(dispatcher.remediation_requests.try_recv(), Some(envelope));
    assert_eq!(dispatcher.remediation_request_stats().accepted, 1);
}

#[test]
fn mismatched_kind_and_payload_is_rejected_before_enqueueing() {
    let mut dispatcher = IpcDispatcher::new(IpcDispatcherConfig::try_new(4).unwrap());
    let mut envelope = sample_alert_envelope();
    envelope.kind = IpcMessageKind::Health;

    let result = dispatcher.dispatch(envelope);

    assert!(matches!(
        result,
        Err(IpcError::MessageKindPayloadMismatch { .. })
    ));
    assert_eq!(dispatcher.alerts.try_recv(), None);
    assert_eq!(dispatcher.rejected_count(), 1);
}

#[test]
fn full_route_queue_returns_queue_full_and_records_drop() {
    let mut dispatcher = IpcDispatcher::new(IpcDispatcherConfig::try_new(1).unwrap());

    dispatcher.dispatch(sample_alert_envelope()).unwrap();
    let result = dispatcher.dispatch(sample_alert_envelope());

    assert!(matches!(result, Err(IpcError::QueueFull { .. })));
    assert_eq!(dispatcher.alert_stats().accepted, 1);
    assert_eq!(dispatcher.alert_stats().dropped, 1);
}

#[test]
fn zero_capacity_dispatcher_config_is_rejected() {
    assert!(matches!(
        IpcDispatcherConfig::try_new(0),
        Err(IpcError::InvalidDispatcherCapacity { capacity: 0 })
    ));
}
