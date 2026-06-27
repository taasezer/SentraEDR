use thiserror::Error;

/// Core telemetry ingestion and pipeline errors.
#[derive(Debug, Error)]
pub enum TelemetryError {
    #[error("ETW Provider Error: {0}")]
    ProviderError(String),
    #[error("Session Error: {0}")]
    SessionError(String),
    #[error("Parse Error: {0}")]
    ParseError(String),
    #[error("Normalization Error: {0}")]
    NormalizationError(String),
}

/// Core engine processing and correlation errors.
#[derive(Debug, Error)]
pub enum EngineError {
    #[error("Detection Error: {0}")]
    DetectionError(String),
    #[error("Persistence Engine Error: {0}")]
    PersistenceError(String),
    #[error("Network Engine Error: {0}")]
    NetworkError(String),
    #[error("Process Engine Error: {0}")]
    ProcessError(String),
}

/// System-level infrastructure errors (queues, serialization, database, IPC).
#[derive(Debug, Error)]
pub enum InfrastructureError {
    #[error("Queue Overflow: {0}")]
    QueueOverflowError(String),
    #[error("Serialization Error: {0}")]
    SerializationError(String),
    #[error("IPC Error: {0}")]
    IpcError(String),
    #[error("Database Error: {0}")]
    DatabaseError(String),
}
