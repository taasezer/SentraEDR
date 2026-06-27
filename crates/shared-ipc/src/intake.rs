use crate::{IpcDispatcher, IpcDispatcherConfig, IpcError, decode_frame};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct IpcFrameIntakeStats {
    pub accepted: u64,
    pub decode_failed: u64,
    pub dispatch_failed: u64,
}

#[derive(Debug)]
pub struct IpcFrameIntake {
    dispatcher: IpcDispatcher,
    stats: IpcFrameIntakeStats,
}

impl IpcFrameIntake {
    pub fn new(config: IpcDispatcherConfig) -> Self {
        Self {
            dispatcher: IpcDispatcher::new(config),
            stats: IpcFrameIntakeStats::default(),
        }
    }

    pub fn accept_frame(&mut self, frame: &[u8]) -> Result<(), IpcError> {
        let envelope = match decode_frame(frame) {
            Ok(envelope) => envelope,
            Err(error) => {
                self.stats.decode_failed += 1;
                return Err(error);
            }
        };

        if let Err(error) = self.dispatcher.dispatch(envelope) {
            self.stats.dispatch_failed += 1;
            return Err(error);
        }

        self.stats.accepted += 1;
        Ok(())
    }

    pub fn stats(&self) -> IpcFrameIntakeStats {
        self.stats
    }

    pub fn dispatcher(&self) -> &IpcDispatcher {
        &self.dispatcher
    }

    pub fn dispatcher_mut(&mut self) -> &mut IpcDispatcher {
        &mut self.dispatcher
    }
}
