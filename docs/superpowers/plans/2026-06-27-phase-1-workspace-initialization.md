# Phase 1 Workspace Initialization Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the first compiling SentraEDR Rust workspace with strict crate boundaries, shared telemetry schemas, a bounded IPC queue primitive, agent config/logging foundations, and architecture validation.

**Architecture:** Phase 1 creates only safe foundations. `shared-models` owns deterministic schemas, `shared-ipc` owns bounded local communication primitives, and `sentra-agent` wires config and logging without starting ETW, remediation, or UI functionality. The plan keeps observe-only defaults and does not implement telemetry collection, named pipes, detection scoring, or remediation.

**Tech Stack:** Rust 1.85+ with edition 2024, Cargo workspace resolver 3, Tokio 1.52.3, Serde 1.0.228, thiserror 2.0.18, uuid 1.23.4, chrono 0.4.45, tracing 0.1.44, tracing-subscriber 0.3.23, toml 1.1.2, tempfile 3.27.0.

---

## Source Inputs

- `docs/superpowers/specs/2026-06-27-phase-0-design.md`
- `ARCHITECTURE.md`
- `SECURITY_MODEL.md`
- `MEMORY_MODEL.md`
- `IPC_DESIGN.md`
- `TASKS.md`
- `PHASE_REPORTS/phase-0.md`

## File Structure

Create:

- `rust-toolchain.toml`: pins stable toolchain channel and required components.
- `.gitignore`: excludes Rust build output and local environment files.
- `Cargo.toml`: root workspace manifest and shared dependency versions.
- `crates/shared-models/Cargo.toml`: shared schema crate manifest.
- `crates/shared-models/src/lib.rs`: schema crate module exports.
- `crates/shared-models/src/time.rs`: UTC timestamp helpers and parse errors.
- `crates/shared-models/src/process.rs`: process identity and lineage models.
- `crates/shared-models/src/telemetry.rs`: normalized telemetry event schema.
- `crates/shared-models/src/detection.rs`: finding and alert schema.
- `crates/shared-models/src/remediation.rs`: remediation command and status schema.
- `crates/shared-models/src/health.rs`: queue and component health schema.
- `crates/shared-models/tests/schema_roundtrip.rs`: serialization and schema invariant tests.
- `crates/shared-ipc/Cargo.toml`: bounded IPC primitive crate manifest.
- `crates/shared-ipc/src/lib.rs`: IPC crate exports.
- `crates/shared-ipc/src/error.rs`: IPC errors.
- `crates/shared-ipc/src/queue.rs`: bounded async queue wrapper and metrics.
- `crates/shared-ipc/tests/bounded_queue.rs`: queue pressure tests.
- `crates/sentra-agent/Cargo.toml`: agent crate manifest.
- `crates/sentra-agent/src/lib.rs`: agent crate exports.
- `crates/sentra-agent/src/config.rs`: safe default configuration and TOML loading.
- `crates/sentra-agent/src/logging.rs`: tracing initialization.
- `crates/sentra-agent/src/main.rs`: minimal CLI entry point.
- `crates/sentra-agent/tests/config.rs`: config parsing tests.
- `tools/validate-architecture.ps1`: dependency boundary validator.
- `TEST_RESULTS/phase-1.md`: Phase 1 verification log template.

Modify:

- `TASKS.md`: mark Phase 1 task progress after implementation.
- `PHASE_REPORTS/phase-1.md`: add Phase 1 report after implementation.

Do not create:

- ETW sessions.
- Windows service installer.
- Tauri UI.
- Named-pipe transport.
- Detection scoring.
- Remediation execution.
- Quarantine storage.

## Task 1: Toolchain And Workspace Root

**Files:**

- Create: `rust-toolchain.toml`
- Create: `.gitignore`
- Create: `Cargo.toml`

- [ ] **Step 1: Verify Rust is unavailable or installed**

Run:

```powershell
rustc --version
cargo --version
```

Expected if Rust is installed:

```text
rustc 1.85.0 or newer
cargo 1.85.0 or newer
```

Expected if Rust is missing:

```text
rustc : The term 'rustc' is not recognized
cargo : The term 'cargo' is not recognized
```

If Rust is missing, install Rust from https://rustup.rs/ and restart the terminal so `rustc` and `cargo` are on `PATH`.

- [ ] **Step 2: Create `rust-toolchain.toml`**

Write:

```toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
profile = "default"
```

- [ ] **Step 3: Create `.gitignore`**

Write:

```gitignore
/target/
/.idea/
/.vscode/
*.pdb
*.log
.env
.env.*
!.env.example
```

- [ ] **Step 4: Create root `Cargo.toml`**

Write:

```toml
[workspace]
resolver = "3"
members = [
    "crates/shared-models",
    "crates/shared-ipc",
    "crates/sentra-agent",
]

[workspace.package]
authors = ["TahaSezer"]
edition = "2024"
license = "MIT"
repository = "https://github.com/taasezer/SentraEDR"
rust-version = "1.85"
version = "0.1.0"

[workspace.dependencies]
chrono = { version = "0.4.45", default-features = false, features = ["clock", "serde"] }
serde = { version = "1.0.228", features = ["derive"] }
serde_json = "1.0.150"
tempfile = "3.27.0"
thiserror = "2.0.18"
tokio = { version = "1.52.3", features = ["macros", "rt-multi-thread", "sync", "time"] }
toml = "1.1.2"
tracing = "0.1.44"
tracing-subscriber = { version = "0.3.23", features = ["env-filter", "fmt"] }
uuid = { version = "1.23.4", features = ["serde", "v4"] }
```

- [ ] **Step 5: Run workspace metadata command**

Run:

```powershell
cargo metadata --no-deps
```

Expected:

```text
error: failed to load manifest for workspace member
```

This failure is expected because member crates are declared before their files exist.

- [ ] **Step 6: Commit workspace root**

Run:

```powershell
git add rust-toolchain.toml .gitignore Cargo.toml
git commit -m "chore: initialize rust workspace root"
```

Expected:

```text
Output contains: chore: initialize rust workspace root
```

## Task 2: Shared Models Crate

**Files:**

- Create: `crates/shared-models/Cargo.toml`
- Create: `crates/shared-models/src/lib.rs`
- Create: `crates/shared-models/src/time.rs`
- Create: `crates/shared-models/src/process.rs`
- Create: `crates/shared-models/src/telemetry.rs`
- Create: `crates/shared-models/src/detection.rs`
- Create: `crates/shared-models/src/remediation.rs`
- Create: `crates/shared-models/src/health.rs`
- Create: `crates/shared-models/tests/schema_roundtrip.rs`

- [ ] **Step 1: Create `crates/shared-models/Cargo.toml`**

Write:

```toml
[package]
name = "shared-models"
authors.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
rust-version.workspace = true
version.workspace = true

[dependencies]
chrono.workspace = true
serde.workspace = true
thiserror.workspace = true
uuid.workspace = true

[dev-dependencies]
serde_json.workspace = true
```

- [ ] **Step 2: Create `crates/shared-models/src/lib.rs`**

Write:

```rust
pub mod detection;
pub mod health;
pub mod process;
pub mod remediation;
pub mod telemetry;
pub mod time;

pub use detection::{Alert, AlertId, Finding, FindingId, RiskLevel, Signal};
pub use health::{ComponentHealth, HealthStatus, QueueHealth};
pub use process::{CommandLine, ImagePath, ProcessIdentity};
pub use remediation::{RemediationAction, RemediationCommand, RemediationMode, RemediationStatus};
pub use telemetry::{
    EventPriority, NormalizedTelemetryEvent, SchemaVersion, TelemetryAction, TelemetryEventId,
    TelemetryMetadata, TelemetrySource,
};
pub use time::{Timestamp, TimestampError};
```

- [ ] **Step 3: Create `crates/shared-models/src/time.rs`**

Write:

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn parse_rfc3339(value: &str) -> Result<Self, TimestampError> {
        let parsed = DateTime::parse_from_rfc3339(value)
            .map_err(|source| TimestampError::InvalidRfc3339 {
                value: value.to_owned(),
                source,
            })?
            .with_timezone(&Utc);
        Ok(Self(parsed))
    }

    pub fn to_rfc3339(&self) -> String {
        self.0.to_rfc3339()
    }
}

#[derive(Debug, Error)]
pub enum TimestampError {
    #[error("invalid RFC3339 timestamp '{value}'")]
    InvalidRfc3339 {
        value: String,
        source: chrono::ParseError,
    },
}

#[cfg(test)]
mod tests {
    use super::Timestamp;

    #[test]
    fn parses_rfc3339_as_utc() {
        let timestamp = Timestamp::parse_rfc3339("2026-06-27T12:00:00+03:00").unwrap();
        assert_eq!(timestamp.to_rfc3339(), "2026-06-27T09:00:00+00:00");
    }
}
```

- [ ] **Step 4: Create `crates/shared-models/src/process.rs`**

Write:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImagePath(String);

impl ImagePath {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandLine(String);

impl CommandLine {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessIdentity {
    pub process_id: u32,
    pub parent_process_id: Option<u32>,
    pub image_path: Option<ImagePath>,
    pub command_line: Option<CommandLine>,
    pub user_sid: Option<String>,
}

impl ProcessIdentity {
    pub fn new(process_id: u32) -> Self {
        Self {
            process_id,
            parent_process_id: None,
            image_path: None,
            command_line: None,
            user_sid: None,
        }
    }

    pub fn with_parent(mut self, parent_process_id: u32) -> Self {
        self.parent_process_id = Some(parent_process_id);
        self
    }

    pub fn with_image_path(mut self, image_path: ImagePath) -> Self {
        self.image_path = Some(image_path);
        self
    }

    pub fn with_command_line(mut self, command_line: CommandLine) -> Self {
        self.command_line = Some(command_line);
        self
    }
}
```

- [ ] **Step 5: Create `crates/shared-models/src/telemetry.rs`**

Write:

```rust
use crate::process::ProcessIdentity;
use crate::time::Timestamp;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaVersion {
    pub major: u16,
    pub minor: u16,
}

impl SchemaVersion {
    pub const V1: Self = Self { major: 1, minor: 0 };
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelemetryEventId(Uuid);

impl TelemetryEventId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TelemetryEventId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EventPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TelemetrySource {
    Etw,
    Sysmon,
    WindowsEventLog,
    InternalHealth,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TelemetryAction {
    ProcessStarted,
    ProcessExited,
    ImageLoaded,
    RegistryChanged,
    PowerShellExecuted,
    NetworkConnectionObserved,
    ComponentHealthChanged,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelemetryMetadata {
    values: BTreeMap<String, String>,
}

impl TelemetryMetadata {
    pub fn empty() -> Self {
        Self {
            values: BTreeMap::new(),
        }
    }

    pub fn insert(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.values.insert(key.into(), value.into());
        self
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }
}

impl Default for TelemetryMetadata {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedTelemetryEvent {
    pub schema_version: SchemaVersion,
    pub event_id: TelemetryEventId,
    pub timestamp: Timestamp,
    pub source: TelemetrySource,
    pub priority: EventPriority,
    pub process: Option<ProcessIdentity>,
    pub action: TelemetryAction,
    pub metadata: TelemetryMetadata,
    pub confidence_hint: u8,
}

impl NormalizedTelemetryEvent {
    pub fn new(
        source: TelemetrySource,
        priority: EventPriority,
        action: TelemetryAction,
        timestamp: Timestamp,
    ) -> Self {
        Self {
            schema_version: SchemaVersion::V1,
            event_id: TelemetryEventId::new(),
            timestamp,
            source,
            priority,
            process: None,
            action,
            metadata: TelemetryMetadata::empty(),
            confidence_hint: 0,
        }
    }

    pub fn with_process(mut self, process: ProcessIdentity) -> Self {
        self.process = Some(process);
        self
    }

    pub fn with_confidence_hint(mut self, confidence_hint: u8) -> Self {
        self.confidence_hint = confidence_hint.min(100);
        self
    }
}
```

- [ ] **Step 6: Create `crates/shared-models/src/detection.rs`**

Write:

```rust
use crate::process::ProcessIdentity;
use crate::telemetry::TelemetryEventId;
use crate::time::Timestamp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FindingId(Uuid);

impl FindingId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for FindingId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlertId(Uuid);

impl AlertId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for AlertId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Informational,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signal {
    pub name: String,
    pub description: String,
    pub supporting_event_ids: Vec<TelemetryEventId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finding {
    pub finding_id: FindingId,
    pub timestamp: Timestamp,
    pub risk_level: RiskLevel,
    pub score: u8,
    pub process: Option<ProcessIdentity>,
    pub signals: Vec<Signal>,
    pub mitre_techniques: Vec<String>,
    pub telemetry_uncertainty: bool,
}

impl Finding {
    pub fn new(timestamp: Timestamp, risk_level: RiskLevel, score: u8) -> Self {
        Self {
            finding_id: FindingId::new(),
            timestamp,
            risk_level,
            score: score.min(100),
            process: None,
            signals: Vec::new(),
            mitre_techniques: Vec::new(),
            telemetry_uncertainty: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Alert {
    pub alert_id: AlertId,
    pub finding: Finding,
    pub recommended_action: String,
    pub remediation_eligible: bool,
}

impl Alert {
    pub fn observe_only(finding: Finding, recommended_action: impl Into<String>) -> Self {
        Self {
            alert_id: AlertId::new(),
            finding,
            recommended_action: recommended_action.into(),
            remediation_eligible: false,
        }
    }
}
```

- [ ] **Step 7: Create `crates/shared-models/src/remediation.rs`**

Write:

```rust
use crate::detection::AlertId;
use crate::time::Timestamp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemediationMode {
    ObserveOnly,
    ApprovalRequired,
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemediationAction {
    SuspendProcess,
    IsolateNetwork,
    QuarantineFile,
    BackupRegistryValue,
    RestoreRegistryValue,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemediationCommand {
    pub command_id: Uuid,
    pub alert_id: AlertId,
    pub requested_at: Timestamp,
    pub requested_by: String,
    pub mode: RemediationMode,
    pub action: RemediationAction,
    pub rationale: String,
}

impl RemediationCommand {
    pub fn new(
        alert_id: AlertId,
        requested_at: Timestamp,
        requested_by: impl Into<String>,
        mode: RemediationMode,
        action: RemediationAction,
        rationale: impl Into<String>,
    ) -> Self {
        Self {
            command_id: Uuid::new_v4(),
            alert_id,
            requested_at,
            requested_by: requested_by.into(),
            mode,
            action,
            rationale: rationale.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemediationStatus {
    RejectedByPolicy,
    WaitingForApproval,
    Approved,
    Completed,
    Failed,
}
```

- [ ] **Step 8: Create `crates/shared-models/src/health.rs`**

Write:

```rust
use crate::time::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Stopped,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueHealth {
    pub name: String,
    pub capacity: usize,
    pub depth: usize,
    pub dropped_events: u64,
}

impl QueueHealth {
    pub fn new(name: impl Into<String>, capacity: usize, depth: usize, dropped_events: u64) -> Self {
        Self {
            name: name.into(),
            capacity,
            depth,
            dropped_events,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub component: String,
    pub status: HealthStatus,
    pub observed_at: Timestamp,
    pub queue: Option<QueueHealth>,
}
```

- [ ] **Step 9: Create `crates/shared-models/tests/schema_roundtrip.rs`**

Write:

```rust
use shared_models::{
    CommandLine, EventPriority, ImagePath, NormalizedTelemetryEvent, ProcessIdentity, RiskLevel,
    TelemetryAction, TelemetrySource, Timestamp,
};

#[test]
fn telemetry_event_roundtrips_through_json() {
    let timestamp = Timestamp::parse_rfc3339("2026-06-27T09:00:00Z").unwrap();
    let process = ProcessIdentity::new(4242)
        .with_parent(1000)
        .with_image_path(ImagePath::new(r"C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe"))
        .with_command_line(CommandLine::new("powershell.exe -NoProfile"));

    let event = NormalizedTelemetryEvent::new(
        TelemetrySource::Etw,
        EventPriority::High,
        TelemetryAction::PowerShellExecuted,
        timestamp,
    )
    .with_process(process)
    .with_confidence_hint(80);

    let encoded = serde_json::to_string(&event).unwrap();
    let decoded: NormalizedTelemetryEvent = serde_json::from_str(&encoded).unwrap();

    assert_eq!(decoded.schema_version.major, 1);
    assert_eq!(decoded.schema_version.minor, 0);
    assert_eq!(decoded.priority, EventPriority::High);
    assert_eq!(decoded.confidence_hint, 80);
}

#[test]
fn finding_score_is_clamped_to_100() {
    let timestamp = Timestamp::parse_rfc3339("2026-06-27T09:00:00Z").unwrap();
    let finding = shared_models::Finding::new(timestamp, RiskLevel::Critical, 200);

    assert_eq!(finding.score, 100);
}
```

- [ ] **Step 10: Run shared-models tests**

Run:

```powershell
cargo test -p shared-models
```

Expected:

```text
test result: ok.
```

- [ ] **Step 11: Commit shared models**

Run:

```powershell
git add crates/shared-models Cargo.toml
git commit -m "feat: add shared telemetry models"
```

Expected:

```text
Output contains: feat: add shared telemetry models
```

## Task 3: Shared IPC Bounded Queue

**Files:**

- Create: `crates/shared-ipc/Cargo.toml`
- Create: `crates/shared-ipc/src/lib.rs`
- Create: `crates/shared-ipc/src/error.rs`
- Create: `crates/shared-ipc/src/queue.rs`
- Create: `crates/shared-ipc/tests/bounded_queue.rs`

- [ ] **Step 1: Create `crates/shared-ipc/Cargo.toml`**

Write:

```toml
[package]
name = "shared-ipc"
authors.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
rust-version.workspace = true
version.workspace = true

[dependencies]
shared-models = { path = "../shared-models" }
thiserror.workspace = true
tokio.workspace = true
```

- [ ] **Step 2: Create `crates/shared-ipc/src/lib.rs`**

Write:

```rust
pub mod error;
pub mod queue;

pub use error::IpcError;
pub use queue::{BoundedReceiver, BoundedSender, QueueSnapshot, bounded_channel};
```

- [ ] **Step 3: Create `crates/shared-ipc/src/error.rs`**

Write:

```rust
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum IpcError {
    #[error("queue '{queue}' is full at capacity {capacity}")]
    QueueFull { queue: String, capacity: usize },

    #[error("queue '{queue}' receiver is closed")]
    ReceiverClosed { queue: String },
}
```

- [ ] **Step 4: Create `crates/shared-ipc/src/queue.rs`**

Write:

```rust
use crate::error::IpcError;
use shared_models::QueueHealth;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use tokio::sync::mpsc;

#[derive(Debug)]
struct QueueMetrics {
    name: String,
    capacity: usize,
    depth: AtomicUsize,
    dropped_events: AtomicU64,
}

impl QueueMetrics {
    fn new(name: impl Into<String>, capacity: usize) -> Self {
        Self {
            name: name.into(),
            capacity,
            depth: AtomicUsize::new(0),
            dropped_events: AtomicU64::new(0),
        }
    }

    fn snapshot(&self) -> QueueSnapshot {
        QueueSnapshot {
            name: self.name.clone(),
            capacity: self.capacity,
            depth: self.depth.load(Ordering::Relaxed),
            dropped_events: self.dropped_events.load(Ordering::Relaxed),
        }
    }

    fn health(&self) -> QueueHealth {
        let snapshot = self.snapshot();
        QueueHealth::new(
            snapshot.name,
            snapshot.capacity,
            snapshot.depth,
            snapshot.dropped_events,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueueSnapshot {
    pub name: String,
    pub capacity: usize,
    pub depth: usize,
    pub dropped_events: u64,
}

#[derive(Debug)]
pub struct BoundedSender<T> {
    sender: mpsc::Sender<T>,
    metrics: Arc<QueueMetrics>,
}

impl<T> Clone for BoundedSender<T> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            metrics: Arc::clone(&self.metrics),
        }
    }
}

impl<T> BoundedSender<T> {
    pub fn try_send(&self, value: T) -> Result<(), IpcError> {
        self.sender.try_send(value).map_err(|error| match error {
            mpsc::error::TrySendError::Full(_) => {
                self.metrics.dropped_events.fetch_add(1, Ordering::Relaxed);
                IpcError::QueueFull {
                    queue: self.metrics.name.clone(),
                    capacity: self.metrics.capacity,
                }
            }
            mpsc::error::TrySendError::Closed(_) => IpcError::ReceiverClosed {
                queue: self.metrics.name.clone(),
            },
        })?;
        self.metrics.depth.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    pub fn snapshot(&self) -> QueueSnapshot {
        self.metrics.snapshot()
    }

    pub fn health(&self) -> QueueHealth {
        self.metrics.health()
    }
}

#[derive(Debug)]
pub struct BoundedReceiver<T> {
    receiver: mpsc::Receiver<T>,
    metrics: Arc<QueueMetrics>,
}

impl<T> BoundedReceiver<T> {
    pub async fn recv(&mut self) -> Option<T> {
        let value = self.receiver.recv().await;
        if value.is_some() {
            self.metrics.depth.fetch_sub(1, Ordering::Relaxed);
        }
        value
    }

    pub fn snapshot(&self) -> QueueSnapshot {
        self.metrics.snapshot()
    }
}

pub fn bounded_channel<T>(
    name: impl Into<String>,
    capacity: usize,
) -> (BoundedSender<T>, BoundedReceiver<T>) {
    assert!(capacity > 0, "bounded channel capacity must be greater than zero");
    let (sender, receiver) = mpsc::channel(capacity);
    let metrics = Arc::new(QueueMetrics::new(name, capacity));

    (
        BoundedSender {
            sender,
            metrics: Arc::clone(&metrics),
        },
        BoundedReceiver { receiver, metrics },
    )
}
```

- [ ] **Step 5: Create `crates/shared-ipc/tests/bounded_queue.rs`**

Write:

```rust
use shared_ipc::{IpcError, bounded_channel};

#[tokio::test]
async fn queue_tracks_depth_after_send_and_receive() {
    let (sender, mut receiver) = bounded_channel("telemetry", 2);

    sender.try_send("first").unwrap();
    assert_eq!(sender.snapshot().depth, 1);

    let received = receiver.recv().await;
    assert_eq!(received, Some("first"));
    assert_eq!(receiver.snapshot().depth, 0);
}

#[tokio::test]
async fn queue_reports_full_without_unbounded_growth() {
    let (sender, _receiver) = bounded_channel("telemetry", 1);

    sender.try_send("first").unwrap();
    let error = sender.try_send("second").unwrap_err();

    assert_eq!(
        error,
        IpcError::QueueFull {
            queue: "telemetry".to_string(),
            capacity: 1,
        }
    );
    assert_eq!(sender.snapshot().depth, 1);
    assert_eq!(sender.snapshot().dropped_events, 1);
}
```

- [ ] **Step 6: Run shared-ipc tests**

Run:

```powershell
cargo test -p shared-ipc
```

Expected:

```text
test result: ok.
```

- [ ] **Step 7: Commit shared IPC**

Run:

```powershell
git add crates/shared-ipc
git commit -m "feat: add bounded ipc queue"
```

Expected:

```text
Output contains: feat: add bounded ipc queue
```

## Task 4: Agent Config And Logging Foundation

**Files:**

- Create: `crates/sentra-agent/Cargo.toml`
- Create: `crates/sentra-agent/src/lib.rs`
- Create: `crates/sentra-agent/src/config.rs`
- Create: `crates/sentra-agent/src/logging.rs`
- Create: `crates/sentra-agent/src/main.rs`
- Create: `crates/sentra-agent/tests/config.rs`

- [ ] **Step 1: Create `crates/sentra-agent/Cargo.toml`**

Write:

```toml
[package]
name = "sentra-agent"
authors.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
rust-version.workspace = true
version.workspace = true

[dependencies]
serde.workspace = true
shared-ipc = { path = "../shared-ipc" }
shared-models = { path = "../shared-models" }
thiserror.workspace = true
tokio.workspace = true
toml.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true

[dev-dependencies]
tempfile.workspace = true
```

- [ ] **Step 2: Create `crates/sentra-agent/src/lib.rs`**

Write:

```rust
pub mod config;
pub mod logging;
```

- [ ] **Step 3: Create `crates/sentra-agent/src/config.rs`**

Write:

```rust
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentConfig {
    pub mode: AgentMode,
    pub queue: QueueConfig,
    pub logging: LoggingConfig,
}

impl AgentConfig {
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        let contents = fs::read_to_string(path).map_err(|source| ConfigError::ReadFailed {
            path: path.display().to_string(),
            source,
        })?;
        toml::from_str(&contents).map_err(|source| ConfigError::ParseFailed {
            path: path.display().to_string(),
            source,
        })
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.queue.telemetry_capacity == 0 {
            return Err(ConfigError::InvalidValue {
                field: "queue.telemetry_capacity",
                reason: "capacity must be greater than zero",
            });
        }

        if self.queue.detection_capacity == 0 {
            return Err(ConfigError::InvalidValue {
                field: "queue.detection_capacity",
                reason: "capacity must be greater than zero",
            });
        }

        Ok(())
    }
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            mode: AgentMode::ObserveOnly,
            queue: QueueConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AgentMode {
    ObserveOnly,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueConfig {
    pub telemetry_capacity: usize,
    pub detection_capacity: usize,
}

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            telemetry_capacity: 4096,
            detection_capacity: 1024,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
        }
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to read config file '{path}'")]
    ReadFailed {
        path: String,
        source: std::io::Error,
    },

    #[error("failed to parse config file '{path}'")]
    ParseFailed {
        path: String,
        source: toml::de::Error,
    },

    #[error("invalid config value for {field}: {reason}")]
    InvalidValue {
        field: &'static str,
        reason: &'static str,
    },
}
```

- [ ] **Step 4: Create `crates/sentra-agent/src/logging.rs`**

Write:

```rust
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt;

pub fn init_logging(level: &str) {
    let filter = EnvFilter::try_new(level).unwrap_or_else(|_| EnvFilter::new("info"));
    let _ = fmt()
        .with_env_filter(filter)
        .with_target(true)
        .try_init();
}
```

- [ ] **Step 5: Create `crates/sentra-agent/src/main.rs`**

Write:

```rust
use sentra_agent::config::AgentConfig;
use sentra_agent::logging::init_logging;
use tracing::info;

fn main() {
    let config = AgentConfig::default();
    init_logging(&config.logging.level);

    if let Err(error) = config.validate() {
        eprintln!("SentraEDR agent configuration error: {error}");
        std::process::exit(2);
    }

    info!(
        mode = ?config.mode,
        telemetry_capacity = config.queue.telemetry_capacity,
        detection_capacity = config.queue.detection_capacity,
        "SentraEDR agent foundation initialized in observe-only mode"
    );
}
```

- [ ] **Step 6: Create `crates/sentra-agent/tests/config.rs`**

Write:

```rust
use sentra_agent::config::{AgentConfig, AgentMode, ConfigError};
use std::fs;

#[test]
fn default_config_is_observe_only() {
    let config = AgentConfig::default();

    assert_eq!(config.mode, AgentMode::ObserveOnly);
    assert_eq!(config.queue.telemetry_capacity, 4096);
    assert_eq!(config.queue.detection_capacity, 1024);
    assert!(config.validate().is_ok());
}

#[test]
fn config_loads_from_toml_file() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("sentra-agent.toml");
    fs::write(
        &path,
        r#"
mode = "observe-only"

[queue]
telemetry_capacity = 128
detection_capacity = 64

[logging]
level = "debug"
"#,
    )
    .unwrap();

    let config = AgentConfig::load_from_file(&path).unwrap();

    assert_eq!(config.mode, AgentMode::ObserveOnly);
    assert_eq!(config.queue.telemetry_capacity, 128);
    assert_eq!(config.queue.detection_capacity, 64);
    assert_eq!(config.logging.level, "debug");
}

#[test]
fn zero_capacity_is_rejected() {
    let mut config = AgentConfig::default();
    config.queue.telemetry_capacity = 0;

    let error = config.validate().unwrap_err();

    assert!(matches!(
        error,
        ConfigError::InvalidValue {
            field: "queue.telemetry_capacity",
            reason: "capacity must be greater than zero"
        }
    ));
}
```

- [ ] **Step 7: Run sentra-agent tests**

Run:

```powershell
cargo test -p sentra-agent
```

Expected:

```text
test result: ok.
```

- [ ] **Step 8: Run the minimal agent binary**

Run:

```powershell
cargo run -p sentra-agent
```

Expected:

```text
SentraEDR agent foundation initialized in observe-only mode
```

The exact log prefix may include timestamp, log level, and target.

- [ ] **Step 9: Commit agent foundation**

Run:

```powershell
git add crates/sentra-agent
git commit -m "feat: add agent config and logging foundation"
```

Expected:

```text
Output contains: feat: add agent config and logging foundation
```

## Task 5: Architecture Boundary Validation

**Files:**

- Create: `tools/validate-architecture.ps1`

- [ ] **Step 1: Create `tools/validate-architecture.ps1`**

Write:

```powershell
$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$cargoFiles = Get-ChildItem -Path (Join-Path $repoRoot "crates") -Filter Cargo.toml -Recurse

$forbidden = @(
    @{ Crate = "shared-models"; Pattern = 'path\s*=\s*"\.\./(shared-ipc|sentra-agent)"'; Message = "shared-models must not depend on other Sentra crates" },
    @{ Crate = "shared-ipc"; Pattern = 'path\s*=\s*"\.\./sentra-agent"'; Message = "shared-ipc must not depend on sentra-agent" },
    @{ Crate = "shared-ipc"; Pattern = 'path\s*=\s*"\.\./engine-'; Message = "shared-ipc must not depend on engine crates" },
    @{ Crate = "sentra-agent"; Pattern = 'path\s*=\s*"\.\./sentra-ui"'; Message = "sentra-agent must not depend on UI crates" }
)

$violations = New-Object System.Collections.Generic.List[string]

foreach ($cargoFile in $cargoFiles) {
    $content = Get-Content -Raw -LiteralPath $cargoFile.FullName
    $crateName = Select-String -InputObject $content -Pattern 'name\s*=\s*"([^"]+)"' | Select-Object -First 1
    if ($null -eq $crateName) {
        $violations.Add("Could not find crate name in $($cargoFile.FullName)")
        continue
    }

    $name = $crateName.Matches[0].Groups[1].Value
    foreach ($rule in $forbidden) {
        if ($name -eq $rule.Crate -and $content -match $rule.Pattern) {
            $violations.Add("$name violates boundary: $($rule.Message)")
        }
    }
}

if ($violations.Count -gt 0) {
    $violations | ForEach-Object { Write-Error $_ }
    exit 1
}

Write-Output "Architecture dependency validation passed."
```

- [ ] **Step 2: Run architecture validation**

Run:

```powershell
powershell -ExecutionPolicy Bypass -File tools/validate-architecture.ps1
```

Expected:

```text
Architecture dependency validation passed.
```

- [ ] **Step 3: Commit architecture validation**

Run:

```powershell
git add tools/validate-architecture.ps1
git commit -m "test: add architecture boundary validation"
```

Expected:

```text
Output contains: test: add architecture boundary validation
```

## Task 6: Workspace Verification And Phase Documentation

**Files:**

- Create: `PHASE_REPORTS/phase-1.md`
- Create: `TEST_RESULTS/phase-1.md`
- Modify: `TASKS.md`

- [ ] **Step 1: Run full formatting check**

Run:

```powershell
cargo fmt --all -- --check
```

Expected:

```text
No output and exit code 0.
```

- [ ] **Step 2: Run clippy**

Run:

```powershell
cargo clippy --workspace --all-targets -- -D warnings
```

Expected:

```text
Finished `dev` profile
```

- [ ] **Step 3: Run all tests**

Run:

```powershell
cargo test --workspace
```

Expected:

```text
test result: ok.
```

- [ ] **Step 4: Run architecture validation**

Run:

```powershell
powershell -ExecutionPolicy Bypass -File tools/validate-architecture.ps1
```

Expected:

```text
Architecture dependency validation passed.
```

- [ ] **Step 5: Create `TEST_RESULTS/phase-1.md`**

Write:

```markdown
# Phase 1 Test Results

Date: 2026-06-27
Phase: Workspace and architecture initialization

## Commands

`cargo fmt --all -- --check`

Result: Passed.

`cargo clippy --workspace --all-targets -- -D warnings`

Result: Passed.

`cargo test --workspace`

Result: Passed.

`powershell -ExecutionPolicy Bypass -File tools/validate-architecture.ps1`

Result: Passed.

## Validation Notes

- The workspace compiles.
- Shared schemas serialize and deserialize through JSON in tests.
- The IPC queue enforces bounded capacity and reports drops.
- The agent defaults to observe-only mode.
- Architecture validation rejects the first set of forbidden dependency directions.
```

- [ ] **Step 6: Create `PHASE_REPORTS/phase-1.md`**

Write:

```markdown
# Phase 1 Report

Date: 2026-06-27
Phase: Workspace and architecture initialization
Status: Complete pending user review

## Active Roles

[ROLE: RUST SYSTEMS ENGINEER]

- Responsibility summary: workspace, crate boundaries, Rust types, and compile hygiene.
- Implementation review: Rust workspace, `shared-models`, `shared-ipc`, and `sentra-agent` were created.
- Validation review: formatting, clippy, tests, and architecture validation passed.
- Concerns: ETW, named pipes, detection scoring, and remediation are not implemented in this phase.
- Approval status: APPROVED FOR USER REVIEW.

[ROLE: CHIEF SECURITY ARCHITECT]

- Responsibility summary: security boundaries and observe-only defaults.
- Implementation review: remediation is represented as schema only and no action executor exists.
- Validation review: default agent mode is observe-only.
- Concerns: future IPC server must enforce pipe ACLs and command authorization.
- Approval status: APPROVED FOR USER REVIEW.

[ROLE: PERFORMANCE ENGINEER]

- Responsibility summary: bounded queues and low-memory design controls.
- Implementation review: `shared-ipc` includes a bounded queue wrapper with drop metrics.
- Validation review: queue capacity and drop behavior are covered by tests.
- Concerns: runtime memory measurements are not available until real telemetry loops exist.
- Approval status: APPROVED FOR USER REVIEW.

[ROLE: QA / VALIDATION ENGINEER]

- Responsibility summary: repeatable validation and phase gate checks.
- Implementation review: test result documentation and architecture validation were added.
- Validation review: all Phase 1 commands passed.
- Concerns: CI automation is still local-only until GitHub Actions is approved.
- Approval status: APPROVED FOR USER REVIEW.

## Completed Work

- Initialized Rust workspace root.
- Added `shared-models`.
- Added `shared-ipc`.
- Added `sentra-agent`.
- Added architecture boundary validation.
- Added Phase 1 verification records.

## Security Impact

The workspace starts in observe-only mode. No remediation executor, named-pipe server, ETW consumer, or privileged service behavior exists yet.

## Performance Impact

The first bounded queue primitive exists and exposes depth and drop metrics. No runtime memory target is claimed yet.

## Telemetry Impact

The normalized telemetry schema exists. No ETW session or telemetry provider is active yet.

## Next Phase

Phase 2 should implement a narrow ETW process-event ingestion path after user approval.

## Human Checkpoint

The user must review Phase 1 before Phase 2 begins. Push to `main` requires separate explicit user approval.
```

- [ ] **Step 7: Update `TASKS.md` Phase 1 section**

Replace the Phase 1 section with:

```markdown
## Phase 1: Workspace And Architecture Initialization

Status: Complete pending user review

Completed:

- Initialized Rust workspace root.
- Added `shared-models` schema crate.
- Added `shared-ipc` bounded queue primitive.
- Added `sentra-agent` config and logging foundation.
- Added architecture dependency validation script.
- Added Phase 1 report and test results.

Validation:

- Formatting passed.
- Clippy passed with warnings denied.
- Workspace tests passed.
- Architecture validation passed.

Architectural impact:

- Phase 0 dependency direction is now represented by crate layout.
- `shared-models` remains dependency-light.
- `shared-ipc` depends on `shared-models` only.
- `sentra-agent` wires foundations without owning detection logic.

Security notes:

- Agent defaults to observe-only mode.
- No remediation executor exists.
- No ETW, named-pipe server, Windows service, or UI exists yet.

Performance notes:

- Bounded queue primitive records depth and dropped events.
- No runtime memory benchmark exists yet.
```

- [ ] **Step 8: Commit Phase 1 documentation**

Run:

```powershell
git add TASKS.md PHASE_REPORTS/phase-1.md TEST_RESULTS/phase-1.md
git commit -m "docs: record phase 1 validation"
```

Expected:

```text
Output contains: docs: record phase 1 validation
```

## Final Verification

- [ ] **Step 1: Confirm git is ahead locally and not pushed**

Run:

```powershell
git status --short --branch
```

Expected:

```text
Output states that local main is ahead of origin/main.
```

- [ ] **Step 2: Confirm commit list**

Run:

```powershell
git log --oneline -8
```

Expected includes:

```text
docs: record phase 1 validation
test: add architecture boundary validation
feat: add agent config and logging foundation
feat: add bounded ipc queue
feat: add shared telemetry models
chore: initialize rust workspace root
docs: define phase 0 architecture
```

## Out Of Scope

The implementer must not add these in Phase 1:

- ETW provider subscriptions.
- Sysmon parser.
- named-pipe server.
- Tauri UI.
- detection scoring.
- process suspension.
- registry modification.
- quarantine.
- malware test execution.

## Self-Review

Spec coverage:

- Phase 1 workspace initialization is covered by Tasks 1-6.
- Crate isolation is covered by Tasks 1, 2, 3, and 5.
- Shared models are covered by Task 2.
- Bounded channels are covered by Task 3.
- Config and logging are covered by Task 4.
- Phase reporting and test results are covered by Task 6.
- Tauri UI, ETW, detection, and remediation are intentionally outside this plan because the approved Phase 0 roadmap assigns them to later phases.

Placeholder scan:

- The plan contains concrete file paths, commands, expected outputs, and full file contents for code steps.

Type consistency:

- `QueueHealth` is exported from `shared-models` and consumed by `shared-ipc`.
- `AgentConfig`, `AgentMode`, and `ConfigError` are defined before tests reference them.
- `Timestamp`, `ProcessIdentity`, `NormalizedTelemetryEvent`, `Finding`, and `RiskLevel` are defined before tests reference them.
