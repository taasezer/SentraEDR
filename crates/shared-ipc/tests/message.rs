use shared_ipc::{IpcEnvelope, IpcError, IpcMessageKind, IpcPayload, MessageId};
use shared_models::{Alert, Finding, RiskLevel, SchemaVersion, Timestamp};

fn sample_timestamp() -> Timestamp {
    Timestamp::parse_rfc3339("2026-06-27T09:00:00Z").unwrap()
}

fn sample_alert() -> Alert {
    let finding = Finding::new(sample_timestamp(), RiskLevel::High, 85);
    Alert::observe_only(finding, "Review suspicious behavior")
}

#[test]
fn alert_envelope_uses_current_schema_version() {
    let envelope = IpcEnvelope::new(
        IpcMessageKind::Alert,
        sample_timestamp(),
        IpcPayload::Alert(sample_alert()),
    )
    .unwrap();

    assert_eq!(envelope.schema_version, SchemaVersion::V1);
    assert_eq!(envelope.kind, IpcMessageKind::Alert);
    assert_eq!(envelope.correlation_id, None);
}

#[test]
fn envelope_can_attach_correlation_id() {
    let correlation_id = MessageId::new();

    let envelope = IpcEnvelope::new(
        IpcMessageKind::Alert,
        sample_timestamp(),
        IpcPayload::Alert(sample_alert()),
    )
    .unwrap()
    .with_correlation_id(correlation_id.clone());

    assert_eq!(envelope.correlation_id, Some(correlation_id));
}

#[test]
fn mismatched_message_kind_and_payload_is_rejected() {
    let result = IpcEnvelope::new(
        IpcMessageKind::Health,
        sample_timestamp(),
        IpcPayload::Alert(sample_alert()),
    );

    assert!(matches!(
        result,
        Err(IpcError::MessageKindPayloadMismatch { .. })
    ));
}

#[test]
fn unsupported_major_schema_version_is_rejected() {
    let mut envelope = IpcEnvelope::new(
        IpcMessageKind::Alert,
        sample_timestamp(),
        IpcPayload::Alert(sample_alert()),
    )
    .unwrap();
    envelope.schema_version = SchemaVersion { major: 2, minor: 0 };

    assert!(matches!(
        envelope.validate(),
        Err(IpcError::UnsupportedSchemaVersion { major: 2, minor: 0 })
    ));
}
