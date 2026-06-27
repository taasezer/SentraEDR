# Phase 2 ETW Process Ingestion Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a testable `engine-etw` crate that normalizes synthetic process start/exit records into shared telemetry events and delivers them through bounded queues.

**Architecture:** Phase 2 is adapter-first and observe-only. `engine-etw` owns process telemetry records, normalization, synthetic event sources, ingestion stats, and bounded queue delivery; `sentra-agent` only runs a synthetic dry run and logs counts. Real Windows ETW callback/session work is deferred until the portable ingestion path is stable.

**Tech Stack:** Rust 1.85+ edition 2024, current workspace crates, Tokio 1.52.3, Serde 1.0.228, thiserror 2.0.18, `stable-x86_64-pc-windows-gnu` for local validation.

---

## Source Inputs

- `docs/superpowers/specs/2026-06-27-phase-2-etw-process-ingestion-design.md`
- `ARCHITECTURE.md`
- `TASKS.md`
- `crates/shared-models/src/telemetry.rs`
- `crates/shared-ipc/src/queue.rs`
- `crates/sentra-agent/src/main.rs`

## File Structure

Create:

- `crates/engine-etw/Cargo.toml`: ETW engine crate manifest.
- `crates/engine-etw/src/lib.rs`: public exports.
- `crates/engine-etw/src/error.rs`: ETW ingestion errors.
- `crates/engine-etw/src/record.rs`: process lifecycle input records.
- `crates/engine-etw/src/normalize.rs`: process record normalization.
- `crates/engine-etw/src/source.rs`: source trait and synthetic source.
- `crates/engine-etw/src/metrics.rs`: ingestion counters and health.
- `crates/engine-etw/src/ingest.rs`: finite-source bounded queue ingestion runner.
- `crates/engine-etw/tests/normalizer.rs`: normalizer tests.
- `crates/engine-etw/tests/ingestion.rs`: bounded ingestion tests.
- `crates/sentra-agent/src/dry_run.rs`: observe-only synthetic ETW dry run.
- `crates/sentra-agent/tests/dry_run.rs`: agent dry-run tests.
- `PHASE_REPORTS/phase-2.md`: Phase 2 report.
- `TEST_RESULTS/phase-2.md`: Phase 2 verification record.

Modify:

- `Cargo.toml`: add `crates/engine-etw`.
- `crates/sentra-agent/Cargo.toml`: depend on `engine-etw`.
- `crates/sentra-agent/src/lib.rs`: export `dry_run`.
- `crates/sentra-agent/src/main.rs`: run synthetic ETW dry run and log counts.
- `tools/validate-architecture.ps1`: add `engine-etw` boundary checks.
- `ARCHITECTURE.md`: mark Phase 2 ETW boundary.
- `PERFORMANCE_NOTES.md`: document synthetic queue pressure validation.
- `TASKS.md`: mark Phase 2 complete after verification.

Do not create:

- Real ETW session lifecycle.
- Windows service installer.
- Named-pipe transport.
- Detection scoring.
- Remediation executor.
- UI streaming.

## Task 1: Engine ETW Normalizer

**Files:**

- Create: `crates/engine-etw/Cargo.toml`
- Create: `crates/engine-etw/src/lib.rs`
- Create: `crates/engine-etw/src/error.rs`
- Create: `crates/engine-etw/src/record.rs`
- Create: `crates/engine-etw/src/normalize.rs`
- Create: `crates/engine-etw/tests/normalizer.rs`
- Modify: `Cargo.toml`

- [ ] **Step 1: Add workspace member and failing normalizer tests**

Create `crates/engine-etw/Cargo.toml`:

```toml
[package]
name = "engine-etw"
authors.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
rust-version.workspace = true
version.workspace = true

[dependencies]
shared-models = { path = "../shared-models" }
shared-ipc = { path = "../shared-ipc" }
thiserror.workspace = true
```

Add `"crates/engine-etw"` to root `Cargo.toml` workspace members.

Create empty `crates/engine-etw/src/lib.rs`.

Create `crates/engine-etw/tests/normalizer.rs`:

```rust
use engine_etw::{EtwProcessEventKind, EtwProcessRecord, normalize_process_record};
use shared_models::{
    EventPriority, TelemetryAction, TelemetrySource, Timestamp,
};

#[test]
fn process_start_record_normalizes_to_telemetry_event() {
    let record = EtwProcessRecord::new(
        EtwProcessEventKind::Start,
        Timestamp::parse_rfc3339("2026-06-27T09:00:00Z").unwrap(),
        4242,
    )
    .with_parent_process_id(1000)
    .with_image_path(r"C:\Windows\System32\cmd.exe")
    .with_command_line("cmd.exe /c whoami")
    .with_confidence(250);

    let event = normalize_process_record(record);
    let process = event.process.unwrap();

    assert_eq!(event.source, TelemetrySource::Etw);
    assert_eq!(event.priority, EventPriority::Medium);
    assert_eq!(event.action, TelemetryAction::ProcessStarted);
    assert_eq!(event.confidence_hint, 100);
    assert_eq!(process.process_id, 4242);
    assert_eq!(process.parent_process_id, Some(1000));
    assert_eq!(process.image_path.unwrap().as_str(), r"C:\Windows\System32\cmd.exe");
    assert_eq!(process.command_line.unwrap().as_str(), "cmd.exe /c whoami");
}

#[test]
fn process_exit_record_normalizes_to_low_priority_exit_event() {
    let record = EtwProcessRecord::new(
        EtwProcessEventKind::Exit,
        Timestamp::parse_rfc3339("2026-06-27T09:01:00Z").unwrap(),
        4242,
    );

    let event = normalize_process_record(record);

    assert_eq!(event.source, TelemetrySource::Etw);
    assert_eq!(event.priority, EventPriority::Low);
    assert_eq!(event.action, TelemetryAction::ProcessExited);
    assert_eq!(event.process.unwrap().process_id, 4242);
}
```

- [ ] **Step 2: Run normalizer tests to verify RED**

Run:

```powershell
cargo test -p engine-etw --test normalizer
```

Expected: compile failure mentioning missing `EtwProcessEventKind`, `EtwProcessRecord`, or `normalize_process_record`.

- [ ] **Step 3: Implement normalizer**

Create `crates/engine-etw/src/lib.rs`:

```rust
pub mod error;
pub mod normalize;
pub mod record;

pub use error::EtwError;
pub use normalize::normalize_process_record;
pub use record::{EtwProcessEventKind, EtwProcessRecord};
```

Create `crates/engine-etw/src/error.rs`:

```rust
use shared_ipc::IpcError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EtwError {
    #[error("ETW source failed: {message}")]
    SourceFailed { message: String },

    #[error("ETW queue delivery failed")]
    QueueDelivery(#[from] IpcError),
}
```

Create `crates/engine-etw/src/record.rs`:

```rust
use shared_models::{CommandLine, ImagePath, Timestamp};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EtwProcessEventKind {
    Start,
    Exit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EtwProcessRecord {
    pub event_kind: EtwProcessEventKind,
    pub timestamp: Timestamp,
    pub process_id: u32,
    pub parent_process_id: Option<u32>,
    pub image_path: Option<ImagePath>,
    pub command_line: Option<CommandLine>,
    pub confidence: u8,
}

impl EtwProcessRecord {
    pub fn new(event_kind: EtwProcessEventKind, timestamp: Timestamp, process_id: u32) -> Self {
        Self {
            event_kind,
            timestamp,
            process_id,
            parent_process_id: None,
            image_path: None,
            command_line: None,
            confidence: 100,
        }
    }

    pub fn with_parent_process_id(mut self, parent_process_id: u32) -> Self {
        self.parent_process_id = Some(parent_process_id);
        self
    }

    pub fn with_image_path(mut self, image_path: impl Into<String>) -> Self {
        self.image_path = Some(ImagePath::new(image_path));
        self
    }

    pub fn with_command_line(mut self, command_line: impl Into<String>) -> Self {
        self.command_line = Some(CommandLine::new(command_line));
        self
    }

    pub fn with_confidence(mut self, confidence: u8) -> Self {
        self.confidence = confidence;
        self
    }
}
```

Create `crates/engine-etw/src/normalize.rs`:

```rust
use crate::record::{EtwProcessEventKind, EtwProcessRecord};
use shared_models::{
    EventPriority, NormalizedTelemetryEvent, ProcessIdentity, TelemetryAction, TelemetryMetadata,
    TelemetrySource,
};

pub fn normalize_process_record(record: EtwProcessRecord) -> NormalizedTelemetryEvent {
    let (priority, action) = match record.event_kind {
        EtwProcessEventKind::Start => (EventPriority::Medium, TelemetryAction::ProcessStarted),
        EtwProcessEventKind::Exit => (EventPriority::Low, TelemetryAction::ProcessExited),
    };

    let process = ProcessIdentity {
        process_id: record.process_id,
        parent_process_id: record.parent_process_id,
        image_path: record.image_path,
        command_line: record.command_line,
        user_sid: None,
    };

    let mut event =
        NormalizedTelemetryEvent::new(TelemetrySource::Etw, priority, action, record.timestamp)
            .with_process(process)
            .with_confidence_hint(record.confidence);
    event.metadata = TelemetryMetadata::empty().insert("engine", "engine-etw");
    event
}
```

- [ ] **Step 4: Run normalizer tests to verify GREEN**

Run:

```powershell
cargo test -p engine-etw --test normalizer
```

Expected: `2 passed`.

- [ ] **Step 5: Commit normalizer**

Run:

```powershell
git add Cargo.toml Cargo.lock crates/engine-etw
git commit -m "feat: add etw process normalizer"
```

Expected output contains `feat: add etw process normalizer`.

## Task 2: Synthetic Source And Bounded Ingestion

**Files:**

- Create: `crates/engine-etw/src/source.rs`
- Create: `crates/engine-etw/src/metrics.rs`
- Create: `crates/engine-etw/src/ingest.rs`
- Create: `crates/engine-etw/tests/ingestion.rs`
- Modify: `crates/engine-etw/src/lib.rs`

- [ ] **Step 1: Write failing ingestion tests**

Create `crates/engine-etw/tests/ingestion.rs`:

```rust
use engine_etw::{
    EtwIngestor, EtwProcessEventKind, EtwProcessRecord, SyntheticEtwSource,
};
use shared_ipc::bounded_channel;
use shared_models::{HealthStatus, NormalizedTelemetryEvent, TelemetryAction, Timestamp};

#[test]
fn synthetic_source_drains_into_bounded_queue() {
    let records = vec![
        EtwProcessRecord::new(
            EtwProcessEventKind::Start,
            Timestamp::parse_rfc3339("2026-06-27T09:00:00Z").unwrap(),
            42,
        ),
        EtwProcessRecord::new(
            EtwProcessEventKind::Exit,
            Timestamp::parse_rfc3339("2026-06-27T09:01:00Z").unwrap(),
            42,
        ),
    ];
    let source = SyntheticEtwSource::from_records(records);
    let (sender, mut receiver) = bounded_channel::<NormalizedTelemetryEvent>("etw-process", 4);

    let report = EtwIngestor::new(source, sender).drain();

    assert_eq!(report.stats.received, 2);
    assert_eq!(report.stats.normalized, 2);
    assert_eq!(report.stats.dropped, 0);
    assert_eq!(report.component_health.status, HealthStatus::Healthy);

    let first = receiver.try_recv().unwrap();
    let second = receiver.try_recv().unwrap();
    assert_eq!(first.action, TelemetryAction::ProcessStarted);
    assert_eq!(second.action, TelemetryAction::ProcessExited);
}

#[test]
fn queue_pressure_degrades_component_health() {
    let records = vec![
        EtwProcessRecord::new(
            EtwProcessEventKind::Start,
            Timestamp::parse_rfc3339("2026-06-27T09:00:00Z").unwrap(),
            1,
        ),
        EtwProcessRecord::new(
            EtwProcessEventKind::Start,
            Timestamp::parse_rfc3339("2026-06-27T09:00:01Z").unwrap(),
            2,
        ),
    ];
    let source = SyntheticEtwSource::from_records(records);
    let (sender, _receiver) = bounded_channel::<NormalizedTelemetryEvent>("etw-process", 1);

    let report = EtwIngestor::new(source, sender).drain();

    assert_eq!(report.stats.received, 2);
    assert_eq!(report.stats.normalized, 1);
    assert_eq!(report.stats.dropped, 1);
    assert_eq!(report.component_health.status, HealthStatus::Degraded);
    assert_eq!(report.component_health.queue.unwrap().dropped_events, 1);
}
```

- [ ] **Step 2: Run ingestion tests to verify RED**

Run:

```powershell
cargo test -p engine-etw --test ingestion
```

Expected: compile failure mentioning missing `EtwIngestor`, `SyntheticEtwSource`, or `try_recv`.

- [ ] **Step 3: Add `try_recv` to shared IPC receiver**

Modify `crates/shared-ipc/src/queue.rs`:

```rust
use crate::error::IpcError;
use shared_models::QueueHealth;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use tokio::sync::mpsc;
```

Add this method to `impl<T> BoundedReceiver<T>`:

```rust
pub fn try_recv(&mut self) -> Option<T> {
    match self.receiver.try_recv() {
        Ok(value) => {
            self.metrics.depth.fetch_sub(1, Ordering::Relaxed);
            Some(value)
        }
        Err(mpsc::error::TryRecvError::Empty | mpsc::error::TryRecvError::Disconnected) => None,
    }
}
```

Add this assertion to `crates/shared-ipc/tests/bounded_queue.rs` in `queue_tracks_depth_after_send_and_receive` before the async receive:

```rust
let received = receiver.try_recv();
assert_eq!(received, Some("first"));
assert_eq!(receiver.snapshot().depth, 0);
```

Remove the later async receive block from that test so it still checks one receive path.

- [ ] **Step 4: Implement source, metrics, and ingestion runner**

Update `crates/engine-etw/src/lib.rs`:

```rust
pub mod error;
pub mod ingest;
pub mod metrics;
pub mod normalize;
pub mod record;
pub mod source;

pub use error::EtwError;
pub use ingest::EtwIngestor;
pub use metrics::{EtwIngestionReport, EtwIngestionStats};
pub use normalize::normalize_process_record;
pub use record::{EtwProcessEventKind, EtwProcessRecord};
pub use source::{EtwEventSource, SyntheticEtwSource};
```

Create `crates/engine-etw/src/source.rs`:

```rust
use crate::error::EtwError;
use crate::record::EtwProcessRecord;
use std::collections::VecDeque;

pub trait EtwEventSource {
    fn next_record(&mut self) -> Result<Option<EtwProcessRecord>, EtwError>;
}

#[derive(Debug, Clone)]
pub struct SyntheticEtwSource {
    records: VecDeque<EtwProcessRecord>,
}

impl SyntheticEtwSource {
    pub fn from_records(records: impl IntoIterator<Item = EtwProcessRecord>) -> Self {
        Self {
            records: records.into_iter().collect(),
        }
    }
}

impl EtwEventSource for SyntheticEtwSource {
    fn next_record(&mut self) -> Result<Option<EtwProcessRecord>, EtwError> {
        Ok(self.records.pop_front())
    }
}
```

Create `crates/engine-etw/src/metrics.rs`:

```rust
use shared_models::{ComponentHealth, HealthStatus, QueueHealth, Timestamp};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EtwIngestionStats {
    pub received: u64,
    pub normalized: u64,
    pub dropped: u64,
    pub failed: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EtwIngestionReport {
    pub stats: EtwIngestionStats,
    pub component_health: ComponentHealth,
}

impl EtwIngestionReport {
    pub fn new(stats: EtwIngestionStats, queue: QueueHealth) -> Self {
        let status = if stats.failed > 0 || stats.dropped > 0 || queue.dropped_events > 0 {
            HealthStatus::Degraded
        } else {
            HealthStatus::Healthy
        };

        Self {
            stats,
            component_health: ComponentHealth {
                component: "engine-etw".to_string(),
                status,
                observed_at: Timestamp::now(),
                queue: Some(queue),
            },
        }
    }
}
```

Create `crates/engine-etw/src/ingest.rs`:

```rust
use crate::metrics::{EtwIngestionReport, EtwIngestionStats};
use crate::normalize::normalize_process_record;
use crate::source::EtwEventSource;
use shared_ipc::BoundedSender;
use shared_models::NormalizedTelemetryEvent;

pub struct EtwIngestor<S> {
    source: S,
    sender: BoundedSender<NormalizedTelemetryEvent>,
}

impl<S> EtwIngestor<S>
where
    S: EtwEventSource,
{
    pub fn new(source: S, sender: BoundedSender<NormalizedTelemetryEvent>) -> Self {
        Self { source, sender }
    }

    pub fn drain(mut self) -> EtwIngestionReport {
        let mut stats = EtwIngestionStats::default();

        loop {
            let record = match self.source.next_record() {
                Ok(Some(record)) => record,
                Ok(None) => break,
                Err(_) => {
                    stats.failed += 1;
                    break;
                }
            };

            stats.received += 1;
            let event = normalize_process_record(record);
            match self.sender.try_send(event) {
                Ok(()) => stats.normalized += 1,
                Err(_) => stats.dropped += 1,
            }
        }

        EtwIngestionReport::new(stats, self.sender.health())
    }
}
```

- [ ] **Step 5: Run queue and ingestion tests to verify GREEN**

Run:

```powershell
cargo test -p shared-ipc
cargo test -p engine-etw
```

Expected: both commands pass.

- [ ] **Step 6: Commit ingestion runner**

Run:

```powershell
git add crates/shared-ipc crates/engine-etw
git commit -m "feat: add etw synthetic ingestion runner"
```

Expected output contains `feat: add etw synthetic ingestion runner`.

## Task 3: Agent Synthetic ETW Dry Run

**Files:**

- Create: `crates/sentra-agent/src/dry_run.rs`
- Create: `crates/sentra-agent/tests/dry_run.rs`
- Modify: `crates/sentra-agent/Cargo.toml`
- Modify: `crates/sentra-agent/src/lib.rs`
- Modify: `crates/sentra-agent/src/main.rs`

- [ ] **Step 1: Write failing dry-run test**

Add `engine-etw = { path = "../engine-etw" }` to `crates/sentra-agent/Cargo.toml`.

Create `crates/sentra-agent/tests/dry_run.rs`:

```rust
use sentra_agent::dry_run::run_synthetic_etw_dry_run;
use shared_models::HealthStatus;

#[test]
fn synthetic_etw_dry_run_reports_two_normalized_events() {
    let report = run_synthetic_etw_dry_run();

    assert_eq!(report.stats.received, 2);
    assert_eq!(report.stats.normalized, 2);
    assert_eq!(report.stats.dropped, 0);
    assert_eq!(report.component_health.status, HealthStatus::Healthy);
}
```

- [ ] **Step 2: Run dry-run test to verify RED**

Run:

```powershell
cargo test -p sentra-agent --test dry_run
```

Expected: compile failure mentioning missing `sentra_agent::dry_run`.

- [ ] **Step 3: Implement dry run**

Modify `crates/sentra-agent/src/lib.rs`:

```rust
pub mod config;
pub mod dry_run;
pub mod logging;
```

Create `crates/sentra-agent/src/dry_run.rs`:

```rust
use engine_etw::{EtwIngestionReport, EtwIngestor, EtwProcessEventKind, EtwProcessRecord, SyntheticEtwSource};
use shared_ipc::bounded_channel;
use shared_models::{NormalizedTelemetryEvent, Timestamp};

pub fn run_synthetic_etw_dry_run() -> EtwIngestionReport {
    let records = vec![
        EtwProcessRecord::new(
            EtwProcessEventKind::Start,
            Timestamp::parse_rfc3339("2026-06-27T09:00:00Z").unwrap(),
            4242,
        )
        .with_parent_process_id(1000)
        .with_image_path(r"C:\Windows\System32\cmd.exe")
        .with_command_line("cmd.exe /c whoami"),
        EtwProcessRecord::new(
            EtwProcessEventKind::Exit,
            Timestamp::parse_rfc3339("2026-06-27T09:01:00Z").unwrap(),
            4242,
        ),
    ];
    let source = SyntheticEtwSource::from_records(records);
    let (sender, _receiver) = bounded_channel::<NormalizedTelemetryEvent>("etw-process-dry-run", 16);

    EtwIngestor::new(source, sender).drain()
}
```

Modify `crates/sentra-agent/src/main.rs` to log the dry-run counts:

```rust
use sentra_agent::config::AgentConfig;
use sentra_agent::dry_run::run_synthetic_etw_dry_run;
use sentra_agent::logging::init_logging;
use tracing::info;

fn main() {
    let config = AgentConfig::default();
    init_logging(&config.logging.level);

    if let Err(error) = config.validate() {
        eprintln!("SentraEDR agent configuration error: {error}");
        std::process::exit(2);
    }

    let report = run_synthetic_etw_dry_run();

    info!(
        mode = ?config.mode,
        telemetry_capacity = config.queue.telemetry_capacity,
        detection_capacity = config.queue.detection_capacity,
        etw_received = report.stats.received,
        etw_normalized = report.stats.normalized,
        etw_dropped = report.stats.dropped,
        "SentraEDR agent foundation initialized in observe-only mode"
    );
}
```

- [ ] **Step 4: Run dry-run tests and binary**

Run:

```powershell
cargo test -p sentra-agent --test dry_run
cargo run -p sentra-agent
```

Expected: test passes and the binary log includes `etw_received=2` and `etw_normalized=2`.

- [ ] **Step 5: Commit agent integration**

Run:

```powershell
git add crates/sentra-agent Cargo.toml Cargo.lock
git commit -m "feat: add agent etw dry run"
```

Expected output contains `feat: add agent etw dry run`.

## Task 4: Architecture Validation And Documentation

**Files:**

- Modify: `tools/validate-architecture.ps1`
- Modify: `ARCHITECTURE.md`
- Modify: `PERFORMANCE_NOTES.md`
- Modify: `TASKS.md`
- Create: `PHASE_REPORTS/phase-2.md`
- Create: `TEST_RESULTS/phase-2.md`

- [ ] **Step 1: Update architecture validation**

Add rules that prevent `engine-etw` from depending on `sentra-agent`, `sentra-ui`, or other `engine-*` crates:

```powershell
@{ Crate = "engine-etw"; Pattern = 'path\s*=\s*"\.\./(sentra-agent|sentra-ui)"'; Message = "engine-etw must not depend on agent or UI crates" },
@{ Crate = "engine-etw"; Pattern = 'path\s*=\s*"\.\./engine-'; Message = "engine-etw must not depend on peer engine crates" }
```

- [ ] **Step 2: Run architecture validation**

Run:

```powershell
powershell -ExecutionPolicy Bypass -File tools/validate-architecture.ps1
```

Expected: `Architecture dependency validation passed.`

- [ ] **Step 3: Update documentation**

Update docs to state:

- Phase 2 added portable `engine-etw` process ingestion.
- Real Windows ETW session/callback remains deferred.
- Synthetic tests validate normalizer, bounded queue delivery, queue pressure, and agent dry run.

Create `TEST_RESULTS/phase-2.md` with the exact verification commands and results after final verification.

Create `PHASE_REPORTS/phase-2.md` with active roles:

- Windows Internals Engineer.
- Rust Systems Engineer.
- Performance Engineer.
- QA / Validation Engineer.
- Documentation Engineer.

- [ ] **Step 4: Commit documentation**

Run:

```powershell
git add tools/validate-architecture.ps1 ARCHITECTURE.md PERFORMANCE_NOTES.md TASKS.md PHASE_REPORTS/phase-2.md TEST_RESULTS/phase-2.md
git commit -m "docs: record phase 2 validation"
```

Expected output contains `docs: record phase 2 validation`.

## Final Verification

- [ ] **Step 1: Run all checks**

Run:

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
powershell -ExecutionPolicy Bypass -File tools/validate-architecture.ps1
cargo run -p sentra-agent
```

Expected:

- format check exits 0;
- clippy exits 0;
- all tests pass;
- architecture validation passes;
- agent dry run logs two received and two normalized synthetic ETW events.

- [ ] **Step 2: Push to Omer**

Run:

```powershell
git push origin Omer
git fetch origin Omer:refs/remotes/origin/Omer
$range = "Omer" + "." + "." + "." + "origin/Omer"
git rev-list --left-right --count $range
```

Expected final count:

```text
0 0
```

## Out Of Scope

- Real ETW kernel session lifecycle.
- `StartTraceW`, `OpenTraceW`, `ProcessTrace` runtime calls.
- Image-load events.
- Registry events.
- PowerShell ETW events.
- Sysmon ingestion.
- Detection scoring.
- Remediation.
- UI or named-pipe IPC.

## Self-Review

Spec coverage:

- `engine-etw` crate: Tasks 1 and 2.
- Process start/exit normalization: Task 1.
- Synthetic event source: Task 2.
- Bounded queue delivery and pressure: Task 2.
- Agent dry run: Task 3.
- Documentation and validation: Task 4.
- Real Windows ETW source: intentionally deferred by Phase 2 spec.

Placeholder scan:

- The plan contains concrete file paths, commands, expected outputs, and code snippets for each behavior-changing step.

Type consistency:

- `EtwProcessRecord` is defined before source and ingestion consume it.
- `EtwIngestionReport` exposes `stats` and `component_health`, matching agent tests.
- `BoundedReceiver::try_recv` is added before ingestion tests call it.
