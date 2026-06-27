//! IPC message types exchanged between SentraEDR components.
//!
//! Every variant is serializable so that messages can be forwarded over named
//! pipes, shared memory, or any other transport that the platform may adopt in
//! the future.

use sentra_core::{
    DetectionResult, RemediationAction, SystemHealth, TelemetryEvent,
};
use serde::{Deserialize, Serialize};

/// Top-level IPC message envelope.
///
/// Components send and receive [`IpcMessage`] values through bounded channels
/// or the [`crate::bus::EventBus`]. Each variant represents a distinct
/// inter-component interaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpcMessage {
    /// A batch of telemetry events collected by the telemetry subsystem.
    TelemetryBatch(Vec<TelemetryEvent>),

    /// The detection engine has identified a threat.
    DetectionAlert(DetectionResult),

    /// A request to the remediation engine to take action.
    RemediationRequest(RemediationAction),

    /// The remediation engine's response to a prior
    /// [`IpcMessage::RemediationRequest`].
    RemediationResponse {
        /// The action that was attempted.
        action: RemediationAction,
        /// Whether the action completed successfully.
        success: bool,
        /// Human-readable details or error message.
        details: String,
    },

    /// Request the system health report from any component that can respond.
    HealthRequest,

    /// Response to a [`IpcMessage::HealthRequest`].
    HealthResponse(SystemHealth),

    /// Instructs all components to perform a graceful shutdown.
    Shutdown,

    /// Instructs all components to reload their configuration from disk.
    ConfigReload,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ensure every variant round-trips through JSON without loss.
    #[test]
    fn ipc_message_serde_roundtrip() {
        let messages: Vec<IpcMessage> = vec![
            IpcMessage::TelemetryBatch(vec![TelemetryEvent::default()]),
            IpcMessage::HealthRequest,
            IpcMessage::Shutdown,
            IpcMessage::ConfigReload,
            IpcMessage::RemediationResponse {
                action: RemediationAction::default(),
                success: true,
                details: "completed".into(),
            },
        ];

        for msg in &messages {
            let json = serde_json::to_string(msg).expect("serialize");
            let _roundtrip: IpcMessage =
                serde_json::from_str(&json).expect("deserialize");
        }
    }
}
