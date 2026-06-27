pub mod dispatcher;
pub mod error;
pub mod frame;
pub mod intake;
pub mod message;
pub mod pipeline;
pub mod queue;
pub mod stream;

pub use dispatcher::{IpcDispatcher, IpcDispatcherConfig, IpcRouteStats};
pub use error::IpcError;
pub use frame::{FRAME_PREFIX_BYTES, MAX_FRAME_PAYLOAD_BYTES, decode_frame, encode_frame};
pub use intake::{IpcFrameIntake, IpcFrameIntakeStats};
pub use message::{
    AuditRecord, IpcEnvelope, IpcMessageKind, IpcPayload, MessageId, RemediationStatusUpdate,
    TelemetrySummary, UserDecision,
};
pub use pipeline::{IpcPipeline, IpcPipelineStats};
pub use queue::{BoundedReceiver, BoundedSender, QueueSnapshot, bounded_channel};
pub use stream::{IpcStreamAssembler, IpcStreamAssemblerStats};
