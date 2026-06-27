use shared_ipc::{
    IpcEnvelope, IpcError, IpcMessageKind, IpcPayload, MAX_FRAME_PAYLOAD_BYTES, decode_frame,
    encode_frame,
};
use shared_models::{Alert, Finding, RiskLevel, Timestamp};

fn sample_timestamp() -> Timestamp {
    Timestamp::parse_rfc3339("2026-06-27T09:00:00Z").unwrap()
}

fn sample_alert() -> Alert {
    let finding = Finding::new(sample_timestamp(), RiskLevel::High, 85);
    Alert::observe_only(finding, "Review suspicious behavior")
}

fn sample_envelope() -> IpcEnvelope {
    IpcEnvelope::new(
        IpcMessageKind::Alert,
        sample_timestamp(),
        IpcPayload::Alert(sample_alert()),
    )
    .unwrap()
}

#[test]
fn alert_envelope_roundtrips_through_frame_codec() {
    let envelope = sample_envelope();

    let encoded = encode_frame(&envelope).unwrap();
    let decoded = decode_frame(&encoded).unwrap();

    assert_eq!(decoded, envelope);
}

#[test]
fn frame_uses_four_byte_big_endian_payload_length() {
    let encoded = encode_frame(&sample_envelope()).unwrap();
    let length_prefix = u32::from_be_bytes(encoded[0..4].try_into().unwrap()) as usize;

    assert_eq!(length_prefix, encoded.len() - 4);
}

#[test]
fn incomplete_frame_is_rejected() {
    let encoded = encode_frame(&sample_envelope()).unwrap();
    let truncated = &encoded[..encoded.len() - 1];

    assert!(matches!(
        decode_frame(truncated),
        Err(IpcError::IncompleteFrame { .. })
    ));
}

#[test]
fn oversized_frame_is_rejected_before_payload_read() {
    let oversized_length = MAX_FRAME_PAYLOAD_BYTES + 1;
    let frame = (oversized_length as u32).to_be_bytes().to_vec();

    assert!(matches!(
        decode_frame(&frame),
        Err(IpcError::FrameTooLarge {
            length,
            max
        }) if length == oversized_length && max == MAX_FRAME_PAYLOAD_BYTES
    ));
}
