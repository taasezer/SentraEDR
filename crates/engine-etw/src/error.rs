use shared_ipc::IpcError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EtwError {
    #[error("ETW source failed: {message}")]
    SourceFailed { message: String },

    #[error("ETW queue delivery failed")]
    QueueDelivery(#[from] IpcError),

    #[error("Windows API Native Error: {0}")]
    NativeError(u32),

    #[error("Malformed ETW Event: {0}")]
    MalformedEvent(String),
}
