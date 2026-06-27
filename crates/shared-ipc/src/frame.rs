use crate::{IpcEnvelope, IpcError};

pub const MAX_FRAME_PAYLOAD_BYTES: usize = 1024 * 1024;
const FRAME_PREFIX_BYTES: usize = 4;

pub fn encode_frame(envelope: &IpcEnvelope) -> Result<Vec<u8>, IpcError> {
    envelope.validate()?;

    let payload =
        serde_json::to_vec(envelope).map_err(|error| IpcError::Serialization(error.to_string()))?;

    if payload.len() > MAX_FRAME_PAYLOAD_BYTES {
        return Err(IpcError::FrameTooLarge {
            length: payload.len(),
            max: MAX_FRAME_PAYLOAD_BYTES,
        });
    }

    let mut frame = Vec::with_capacity(FRAME_PREFIX_BYTES + payload.len());
    frame.extend_from_slice(&(payload.len() as u32).to_be_bytes());
    frame.extend_from_slice(&payload);
    Ok(frame)
}

pub fn decode_frame(frame: &[u8]) -> Result<IpcEnvelope, IpcError> {
    if frame.len() < FRAME_PREFIX_BYTES {
        return Err(IpcError::IncompleteFrame {
            expected: FRAME_PREFIX_BYTES,
            actual: frame.len(),
        });
    }

    let payload_length = u32::from_be_bytes(
        frame[0..FRAME_PREFIX_BYTES]
            .try_into()
            .expect("slice length is checked"),
    ) as usize;

    if payload_length > MAX_FRAME_PAYLOAD_BYTES {
        return Err(IpcError::FrameTooLarge {
            length: payload_length,
            max: MAX_FRAME_PAYLOAD_BYTES,
        });
    }

    let expected_length = FRAME_PREFIX_BYTES + payload_length;
    if frame.len() < expected_length {
        return Err(IpcError::IncompleteFrame {
            expected: expected_length,
            actual: frame.len(),
        });
    }

    if frame.len() != expected_length {
        return Err(IpcError::Serialization(
            "frame contains trailing bytes".to_owned(),
        ));
    }

    let envelope: IpcEnvelope = serde_json::from_slice(&frame[FRAME_PREFIX_BYTES..])
        .map_err(|error| IpcError::Serialization(error.to_string()))?;
    envelope.validate()?;
    Ok(envelope)
}
