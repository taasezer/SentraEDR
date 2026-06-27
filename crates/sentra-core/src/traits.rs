//! Core traits that define the EDR processing pipeline.
//!
//! These traits are the integration boundaries between crates:
//!
//! - **[`TelemetrySource`]** — implemented by sensor crates
//!   (`sentra-telemetry`, `sentra-process`, etc.) to feed events into
//!   the pipeline.
//! - **[`Detector`]** — implemented by `sentra-detection` rules to
//!   analyse events and emit [`DetectionResult`]s.
//! - **[`Remediator`]** — implemented by `sentra-remediation` to
//!   execute response actions.
//! - **[`HealthReporter`]** — implemented by any component that
//!   exposes health metrics.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::types::{
    DetectionResult, RemediationAction, SystemHealth, TelemetryEvent,
};

// ---------------------------------------------------------------------------
// Telemetry Source
// ---------------------------------------------------------------------------

/// A source of telemetry events (e.g., ETW provider, process poller).
///
/// Implementations are expected to push [`TelemetryEvent`]s into a
/// bounded channel provided at construction time.
pub trait TelemetrySource: Send {
    /// Start the source. This may spawn background tasks.
    fn start(&mut self) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Gracefully stop the source and release resources.
    fn stop(&mut self) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Human-readable name used in logs and health reports.
    fn name(&self) -> &str;
}

// ---------------------------------------------------------------------------
// Detector
// ---------------------------------------------------------------------------

/// A detection rule that inspects a single [`TelemetryEvent`].
///
/// Rules are intentionally **synchronous** — they must complete in
/// bounded time so the event pipeline is never blocked.
pub trait Detector: Send + Sync {
    /// Human-readable name of this detection rule.
    fn name(&self) -> &str;

    /// Analyse `event` and return a [`DetectionResult`] if the rule
    /// fires, or `None` if the event is benign from this rule's
    /// perspective.
    fn analyze(&self, event: &TelemetryEvent) -> Option<DetectionResult>;

    /// MITRE ATT&CK categories this rule covers.
    fn threat_categories(&self) -> Vec<String>;
}

// ---------------------------------------------------------------------------
// Remediation
// ---------------------------------------------------------------------------

/// Outcome of executing a [`RemediationAction`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationOutcome {
    /// Whether the action succeeded.
    pub success: bool,
    /// The action that was executed.
    pub action: RemediationAction,
    /// Human-readable details / error message.
    pub details: String,
    /// When the action completed.
    pub timestamp: DateTime<Utc>,
}

/// Executor for remediation actions.
pub trait Remediator: Send + Sync {
    /// Execute the given remediation action.
    fn execute(
        &self,
        action: &RemediationAction,
    ) -> impl std::future::Future<Output = Result<RemediationOutcome>> + Send;

    /// Returns `true` if this remediator can handle `action`.
    fn supports(&self, action: &RemediationAction) -> bool;
}

// ---------------------------------------------------------------------------
// Health
// ---------------------------------------------------------------------------

/// Any component that can report its current health.
pub trait HealthReporter: Send + Sync {
    /// Return a snapshot of current health metrics.
    fn health(&self) -> SystemHealth;
}
