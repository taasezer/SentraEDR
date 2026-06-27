//! Error types for the SentraEDR platform.
//!
//! Provides a unified [`SentraError`] enum covering all failure modes
//! encountered across the EDR pipeline — from Windows API calls to
//! configuration parsing, channel failures, and remediation errors.

use thiserror::Error;

/// Unified error type for SentraEDR operations.
///
/// Every crate in the workspace propagates errors through this type,
/// ensuring consistent error handling and structured logging across
/// the entire EDR pipeline.
#[derive(Debug, Error)]
pub enum SentraError {
    /// A Windows API call failed.
    #[error("Windows API error: {0}")]
    WindowsApi(String),

    /// Configuration parsing or validation failed.
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// An internal channel (bounded sender/receiver) operation failed.
    #[error("Channel error: {0}")]
    Channel(String),

    /// A detection rule or engine encountered an error.
    #[error("Detection error: {0}")]
    Detection(String),

    /// A remediation action failed to execute.
    #[error("Remediation error: {0}")]
    Remediation(String),

    /// A telemetry source encountered an error.
    #[error("Telemetry error: {0}")]
    Telemetry(String),

    /// An I/O operation failed (transparently wraps [`std::io::Error`]).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization or deserialization failed.
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// The caller lacks required permissions.
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// A resource limit (memory, handles, queue depth) was exceeded.
    #[error("Resource exhausted: {0}")]
    ResourceExhausted(String),
}

/// Convenience alias used throughout the SentraEDR codebase.
pub type Result<T> = std::result::Result<T, SentraError>;
