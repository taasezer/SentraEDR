use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum IpcError {
    #[error("queue '{queue}' is full at capacity {capacity}")]
    QueueFull { queue: String, capacity: usize },

    #[error("queue '{queue}' receiver is closed")]
    ReceiverClosed { queue: String },

    #[error("unsupported schema version {major}.{minor}")]
    UnsupportedSchemaVersion { major: u16, minor: u16 },

    #[error("message kind '{kind}' does not match payload '{payload}'")]
    MessageKindPayloadMismatch { kind: String, payload: String },
}
