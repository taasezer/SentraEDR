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

    #[error("frame payload length {length} exceeds maximum {max}")]
    FrameTooLarge { length: usize, max: usize },

    #[error("incomplete frame: expected {expected} bytes, got {actual}")]
    IncompleteFrame { expected: usize, actual: usize },

    #[error("serialization error: {0}")]
    Serialization(String),

    #[error("dispatcher queue capacity must be greater than zero, got {capacity}")]
    InvalidDispatcherCapacity { capacity: usize },
}
