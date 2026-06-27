use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum IpcError {
    #[error("queue '{queue}' is full at capacity {capacity}")]
    QueueFull { queue: String, capacity: usize },

    #[error("queue '{queue}' receiver is closed")]
    ReceiverClosed { queue: String },
}
