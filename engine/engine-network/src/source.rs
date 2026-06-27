use crate::errors::NetworkEngineError;
use crate::models::{ConnectionSnapshot, ConnectionStateChange};

/// Generic abstraction for Network telemetry (ETW, WinDivert, Kernel Driver).
pub trait NetworkProvider {
    fn provider_name(&self) -> &str;

    /// Establishes the asynchronous stream or callback for network events.
    /// In this phase, we mock this interface to ensure transport decoupling.
    fn start_capture(&self) -> Result<(), NetworkEngineError>;

    /// Gracefully stops the packet/event capture.
    fn stop_capture(&self) -> Result<(), NetworkEngineError>;
}
