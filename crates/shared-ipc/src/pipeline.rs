use crate::intake::IpcFrameIntake;
use crate::stream::IpcStreamAssembler;
use crate::{IpcDispatcherConfig, IpcError};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct IpcPipelineStats {
    pub chunks_received: u64,
    pub frames_completed: u64,
    pub frames_accepted: u64,
    pub stream_rejected: u64,
    pub intake_decode_failed: u64,
    pub intake_dispatch_failed: u64,
}

pub struct IpcPipeline {
    assembler: IpcStreamAssembler,
    intake: IpcFrameIntake,
    stats: IpcPipelineStats,
}

impl IpcPipeline {
    pub fn new(config: IpcDispatcherConfig) -> Self {
        Self {
            assembler: IpcStreamAssembler::new(),
            intake: IpcFrameIntake::new(config),
            stats: IpcPipelineStats::default(),
        }
    }

    pub fn process_bytes(&mut self, chunk: &[u8]) -> Result<(), IpcError> {
        self.stats.chunks_received += 1;

        let frames = match self.assembler.push_bytes(chunk) {
            Ok(frames) => frames,
            Err(error) => {
                self.stats.stream_rejected += 1;
                return Err(error);
            }
        };

        for frame in frames {
            self.stats.frames_completed += 1;
            match self.intake.accept_frame(&frame) {
                Ok(()) => {
                    self.stats.frames_accepted += 1;
                }
                Err(error) => {
                    // Determine if it's a decode error or dispatch error
                    // Based on IpcFrameIntake implementation:
                    // decode errors are returned from decode_frame
                    // dispatch errors are returned from dispatcher.dispatch

                    // We can check the internal stats of the intake to be sure,
                    // but let's just update based on the result of accept_frame.
                    // Actually, IpcFrameIntake already updates its own stats.
                    // Let's sync them or just rely on the error type.

                    // If it's a decode error, it's usually Serialization or something.
                    // If it's a dispatch error, it's usually QueueFull.

                    // To be precise, let's check which one incremented in the intake stats.
                    // This is slightly tricky because other frames in the same chunk might have also failed.
                    // Better approach: check the error variant.

                    match error {
                        IpcError::Serialization(_)
                        | IpcError::UnsupportedSchemaVersion { .. }
                        | IpcError::MessageKindPayloadMismatch { .. } => {
                            self.stats.intake_decode_failed += 1;
                        }
                        IpcError::QueueFull { .. } | IpcError::ReceiverClosed { .. } => {
                            self.stats.intake_dispatch_failed += 1;
                        }
                        _ => {
                            // Fallback for other errors
                            self.stats.intake_dispatch_failed += 1;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn stats(&self) -> IpcPipelineStats {
        self.stats
    }

    pub fn dispatcher(&self) -> &crate::dispatcher::IpcDispatcher {
        self.intake.dispatcher()
    }

    pub fn dispatcher_mut(&mut self) -> &mut crate::dispatcher::IpcDispatcher {
        self.intake.dispatcher_mut()
    }
}
