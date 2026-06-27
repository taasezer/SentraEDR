use thiserror::Error;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Error)]
pub enum LifecycleError {
    #[error("Initialization Failed: {0}")]
    InitFailed(String),
    #[error("Startup Budget Exceeded")]
    StartupTimeout,
    #[error("Shutdown Budget Exceeded")]
    ShutdownTimeout,
}

#[async_trait::async_trait]
pub trait Service: Send + Sync {
    async fn initialize(&mut self) -> Result<(), LifecycleError>;

    /// Must accept a cloned CancellationToken. The service is expected to await `token.cancelled()`
    /// to initiate graceful teardown.
    async fn start(&mut self, token: CancellationToken) -> Result<(), LifecycleError>;

    async fn stop(&mut self) -> Result<(), LifecycleError>;
    async fn shutdown(&mut self) -> Result<(), LifecycleError>;
}

pub struct Supervisor {
    // Handles restart policies, panic isolation, and exponential backoff
}
