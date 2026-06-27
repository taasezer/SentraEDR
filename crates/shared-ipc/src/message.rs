use crate::IpcError;
use serde::{Deserialize, Serialize};
use shared_models::{
    Alert, AlertId, ComponentHealth, EventPriority, RemediationCommand, RemediationStatus,
    SchemaVersion, TelemetrySource, Timestamp,
};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageId(Uuid);

impl MessageId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for MessageId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IpcMessageKind {
    Health,
    TelemetrySummary,
    Alert,
    UserDecision,
    RemediationRequest,
    RemediationStatus,
    AuditRecord,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelemetrySummary {
    pub generated_at: Timestamp,
    pub source: TelemetrySource,
    pub event_count: u64,
    pub highest_priority: EventPriority,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserDecision {
    pub alert_id: AlertId,
    pub decided_at: Timestamp,
    pub decided_by: String,
    pub approved: bool,
    pub rationale: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemediationStatusUpdate {
    pub command_id: Uuid,
    pub observed_at: Timestamp,
    pub status: RemediationStatus,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditRecord {
    pub audit_id: Uuid,
    pub recorded_at: Timestamp,
    pub actor: String,
    pub action: String,
    pub outcome: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IpcPayload {
    Health(ComponentHealth),
    TelemetrySummary(TelemetrySummary),
    Alert(Alert),
    UserDecision(UserDecision),
    RemediationRequest(RemediationCommand),
    RemediationStatus(RemediationStatusUpdate),
    AuditRecord(AuditRecord),
}

impl IpcPayload {
    pub fn kind(&self) -> IpcMessageKind {
        match self {
            Self::Health(_) => IpcMessageKind::Health,
            Self::TelemetrySummary(_) => IpcMessageKind::TelemetrySummary,
            Self::Alert(_) => IpcMessageKind::Alert,
            Self::UserDecision(_) => IpcMessageKind::UserDecision,
            Self::RemediationRequest(_) => IpcMessageKind::RemediationRequest,
            Self::RemediationStatus(_) => IpcMessageKind::RemediationStatus,
            Self::AuditRecord(_) => IpcMessageKind::AuditRecord,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IpcEnvelope {
    pub schema_version: SchemaVersion,
    pub message_id: MessageId,
    pub kind: IpcMessageKind,
    pub timestamp: Timestamp,
    pub correlation_id: Option<MessageId>,
    pub payload: IpcPayload,
}

impl IpcEnvelope {
    pub fn new(
        kind: IpcMessageKind,
        timestamp: Timestamp,
        payload: IpcPayload,
    ) -> Result<Self, IpcError> {
        let envelope = Self {
            schema_version: SchemaVersion::V1,
            message_id: MessageId::new(),
            kind,
            timestamp,
            correlation_id: None,
            payload,
        };
        envelope.validate()?;
        Ok(envelope)
    }

    pub fn with_correlation_id(mut self, correlation_id: MessageId) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    pub fn validate(&self) -> Result<(), IpcError> {
        if self.schema_version.major != SchemaVersion::V1.major {
            return Err(IpcError::UnsupportedSchemaVersion {
                major: self.schema_version.major,
                minor: self.schema_version.minor,
            });
        }

        let payload_kind = self.payload.kind();
        if self.kind != payload_kind {
            return Err(IpcError::MessageKindPayloadMismatch {
                kind: format!("{:?}", self.kind),
                payload: format!("{:?}", payload_kind),
            });
        }

        Ok(())
    }
}
