pub mod error;
pub mod message;
pub mod queue;

pub use error::IpcError;
pub use message::{
    AuditRecord, IpcEnvelope, IpcMessageKind, IpcPayload, MessageId, RemediationStatusUpdate,
    TelemetrySummary, UserDecision,
};
pub use queue::{BoundedReceiver, BoundedSender, QueueSnapshot, bounded_channel};
