use shared_ipc::{
    FRAME_PREFIX_BYTES, IpcEnvelope, IpcError, IpcMessageKind, IpcPayload, IpcStreamAssembler,
    MAX_FRAME_PAYLOAD_BYTES, encode_frame,
};
use shared_models::{Alert, Finding, RiskLevel, Timestamp};

fn sample_timestamp() -> Timestamp {
    Timestamp::parse_rfc3339("2026-06-27T09:00:00Z").unwrap()
}

fn sample_alert_envelope(score: u8) -> IpcEnvelope {
    let finding = Finding::new(sample_timestamp(), RiskLevel::High, score);
    let alert = Alert::observe_only(finding, "Review suspicious behavior");

    IpcEnvelope::new(
        IpcMessageKind::Alert,
        sample_timestamp(),
        IpcPayload::Alert(alert),
    )
    .unwrap()
}

#[test]
fn split_frame_is_emitted_only_after_complete_chunk_arrives() {
    let mut assembler = IpcStreamAssembler::new();
    let frame = encode_frame(&sample_alert_envelope(80)).unwrap();
    let split_at = FRAME_PREFIX_BYTES + 2;

    let first = assembler.push_bytes(&frame[..split_at]).unwrap();
    assert!(first.is_empty());
    assert_eq!(assembler.stats().bytes_buffered, split_at);

    let second = assembler.push_bytes(&frame[split_at..]).unwrap();
    assert_eq!(second, vec![frame]);
    assert_eq!(assembler.stats().frames_completed, 1);
    assert_eq!(assembler.stats().bytes_buffered, 0);
}

#[test]
fn two_complete_frames_in_one_chunk_are_both_emitted() {
    let mut assembler = IpcStreamAssembler::new();
    let first = encode_frame(&sample_alert_envelope(70)).unwrap();
    let second = encode_frame(&sample_alert_envelope(90)).unwrap();
    let mut chunk = first.clone();
    chunk.extend_from_slice(&second);

    let frames = assembler.push_bytes(&chunk).unwrap();

    assert_eq!(frames, vec![first, second]);
    assert_eq!(assembler.stats().frames_completed, 2);
    assert_eq!(assembler.stats().bytes_buffered, 0);
}

#[test]
fn oversized_length_prefix_is_rejected_before_payload_buffering() {
    let mut assembler = IpcStreamAssembler::new();
    let oversized = MAX_FRAME_PAYLOAD_BYTES + 1;
    let frame_prefix = (oversized as u32).to_be_bytes();

    let result = assembler.push_bytes(&frame_prefix);

    assert!(matches!(
        result,
        Err(IpcError::FrameTooLarge { length, max })
            if length == oversized && max == MAX_FRAME_PAYLOAD_BYTES
    ));
    assert_eq!(assembler.stats().rejected, 1);
    assert_eq!(assembler.stats().bytes_buffered, 0);
}

#[test]
fn partial_prefix_bytes_remain_buffered() {
    let mut assembler = IpcStreamAssembler::new();

    let frames = assembler.push_bytes(&[0, 0]).unwrap();

    assert!(frames.is_empty());
    assert_eq!(assembler.stats().frames_completed, 0);
    assert_eq!(assembler.stats().bytes_buffered, 2);
}
