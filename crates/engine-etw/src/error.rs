use shared_ipc::IpcError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EtwError {
    #[error("ETW source failed: {message}")]
    SourceFailed { message: String },

    #[error("ETW queue delivery failed")]
    QueueDelivery(#[from] IpcError),
}
