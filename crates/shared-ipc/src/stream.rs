use crate::{FRAME_PREFIX_BYTES, IpcError, MAX_FRAME_PAYLOAD_BYTES};

const MAX_BUFFERED_BYTES: usize = FRAME_PREFIX_BYTES + MAX_FRAME_PAYLOAD_BYTES;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct IpcStreamAssemblerStats {
    pub frames_completed: u64,
    pub bytes_buffered: usize,
    pub rejected: u64,
}

#[derive(Debug, Default)]
pub struct IpcStreamAssembler {
    buffer: Vec<u8>,
    stats: IpcStreamAssemblerStats,
}

impl IpcStreamAssembler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_bytes(&mut self, chunk: &[u8]) -> Result<Vec<Vec<u8>>, IpcError> {
        if self.buffer.len() + chunk.len() > MAX_BUFFERED_BYTES {
            self.stats.rejected += 1;
            return Err(IpcError::StreamBufferTooLarge {
                length: self.buffer.len() + chunk.len(),
                max: MAX_BUFFERED_BYTES,
            });
        }

        self.buffer.extend_from_slice(chunk);
        let mut frames = Vec::new();

        loop {
            if self.buffer.len() < FRAME_PREFIX_BYTES {
                break;
            }

            let payload_length = u32::from_be_bytes(
                self.buffer[0..FRAME_PREFIX_BYTES]
                    .try_into()
                    .expect("slice length is checked"),
            ) as usize;

            if payload_length > MAX_FRAME_PAYLOAD_BYTES {
                self.buffer.clear();
                self.stats.rejected += 1;
                self.stats.bytes_buffered = 0;
                return Err(IpcError::FrameTooLarge {
                    length: payload_length,
                    max: MAX_FRAME_PAYLOAD_BYTES,
                });
            }

            let frame_length = FRAME_PREFIX_BYTES + payload_length;
            if self.buffer.len() < frame_length {
                break;
            }

            let frame: Vec<u8> = self.buffer.drain(..frame_length).collect();
            frames.push(frame);
            self.stats.frames_completed += 1;
        }

        self.stats.bytes_buffered = self.buffer.len();
        Ok(frames)
    }

    pub fn stats(&self) -> IpcStreamAssemblerStats {
        self.stats
    }
}
