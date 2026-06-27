use std::sync::Arc;
use tokio_util::sync::CancellationToken;
use core_runtime::clock::Clock;

pub struct RuleExecutionContext {
    pub clock: Arc<dyn Clock>,
    // pub config: Arc<LayeredConfiguration>,
    // pub logger: Arc<dyn Logger>,
    pub cancellation_token: CancellationToken,
}

impl RuleExecutionContext {
    pub fn new(clock: Arc<dyn Clock>, token: CancellationToken) -> Self {
        Self {
            clock,
            cancellation_token: token,
        }
    }
}
