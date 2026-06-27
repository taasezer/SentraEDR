use crate::config::IpcConfig;
use shared_ipc::{IpcDispatcher, IpcDispatcherConfig, IpcError, IpcPipeline, IpcPipelineStats};

pub struct IpcService {
    enabled: bool,
    pipeline: IpcPipeline,
}

impl IpcService {
    pub fn new(config: IpcConfig) -> Result<Self, IpcError> {
        let dispatcher_config = IpcDispatcherConfig::try_new(config.dispatcher_capacity)?;

        Ok(Self {
            enabled: config.enabled,
            pipeline: IpcPipeline::new(dispatcher_config),
        })
    }

    pub fn process_raw_bytes(&mut self, chunk: &[u8]) -> Result<(), IpcError> {
        if !self.enabled {
            return Ok(());
        }

        self.pipeline.process_bytes(chunk)
    }

    pub fn stats(&self) -> IpcPipelineStats {
        self.pipeline.stats()
    }

    pub fn dispatcher(&self) -> &IpcDispatcher {
        self.pipeline.dispatcher()
    }

    pub fn dispatcher_mut(&mut self) -> &mut IpcDispatcher {
        self.pipeline.dispatcher_mut()
    }
}
