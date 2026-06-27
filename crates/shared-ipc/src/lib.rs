pub mod dispatcher;
pub mod error;
pub mod frame;
pub mod intake;
pub mod message;
pub mod queue;

pub use dispatcher::{IpcDispatcher, IpcDispatcherConfig, IpcRouteStats};
pub use error::IpcError;
pub use frame::{MAX_FRAME_PAYLOAD_BYTES, decode_frame, encode_frame};
pub use intake::{IpcFrameIntake, IpcFrameIntakeStats};
pub use message::{
    AuditRecord, IpcEnvelope, IpcMessageKind, IpcPayload, MessageId, RemediationStatusUpdate,
    TelemetrySummary, UserDecision,
};
pub use queue::{BoundedReceiver, BoundedSender, QueueSnapshot, bounded_channel};
