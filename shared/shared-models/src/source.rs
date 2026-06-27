use crate::errors::TelemetryError;
use crate::events::NormalizedTelemetryEvent;

/// A generic abstraction for telemetry ingestion.
/// Ensures the detection pipeline remains entirely independent of ETW or Sysmon specifics.
pub trait TelemetrySource {
    /// Start the telemetry source.
    fn start(&mut self) -> Result<(), TelemetryError>;

    /// Stop the telemetry source gracefully.
    fn stop(&mut self) -> Result<(), TelemetryError>;

    /// Returns the name of the telemetry provider (e.g., "Windows ETW", "Sysmon").
    fn name(&self) -> &str;
}
