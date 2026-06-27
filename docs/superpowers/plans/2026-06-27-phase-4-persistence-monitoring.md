# Phase 4 Persistence Monitoring Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add `engine-persistence`, a lightweight observe-only persistence analyzer that consumes normalized telemetry metadata and emits preliminary persistence behavior signals.

**Architecture:** `engine-persistence` consumes `shared-models::NormalizedTelemetryEvent` only and reads persistence details from `TelemetryMetadata`. It owns metadata parsing, deterministic signal matching, and analysis reports; `sentra-agent` only runs a synthetic dry run and logs counts.

**Tech Stack:** Rust 1.85+ edition 2024, existing workspace crates, `shared-models`, `stable-x86_64-pc-windows-gnu` for local validation.

---

## Source Inputs

- `docs/superpowers/specs/2026-06-27-phase-4-persistence-monitoring-design.md`
- `crates/shared-models/src/telemetry.rs`
- `crates/shared-models/src/health.rs`
- `crates/sentra-agent/src/main.rs`
- `crates/sentra-agent/src/process_dry_run.rs`
- `tools/validate-architecture.ps1`

## File Structure

Create:

- `crates/engine-persistence/Cargo.toml`: persistence engine crate manifest.
- `crates/engine-persistence/src/lib.rs`: public exports.
- `crates/engine-persistence/src/event.rs`: metadata parser and `PersistenceEvent` model.
- `crates/engine-persistence/src/signal.rs`: persistence signal types and deterministic matchers.
- `crates/engine-persistence/src/analyzer.rs`: telemetry-to-report orchestration.
- `crates/engine-persistence/tests/analyzer.rs`: core analyzer and signal tests.
- `crates/sentra-agent/src/persistence_dry_run.rs`: synthetic persistence analysis dry run.
- `crates/sentra-agent/tests/persistence_dry_run.rs`: agent synthetic persistence analysis test.
- `PHASE_REPORTS/phase-4.md`: Phase 4 report.
- `TEST_RESULTS/phase-4.md`: Phase 4 verification record.

Modify:

- `Cargo.toml`: add `crates/engine-persistence`.
- `crates/sentra-agent/Cargo.toml`: depend on `engine-persistence`.
- `crates/sentra-agent/src/lib.rs`: export `persistence_dry_run`.
- `crates/sentra-agent/src/main.rs`: log persistence analysis dry-run counts.
- `tools/validate-architecture.ps1`: add `engine-persistence` boundary checks.
- `ARCHITECTURE.md`: mark Phase 4 persistence engine boundary.
- `PERFORMANCE_NOTES.md`: record synthetic metadata/signal validation.
- `TASKS.md`: mark Phase 4 complete after verification.

Do not create:

- Windows Registry API access;
- scheduled task enumeration or task XML parsing;
- Service Control Manager access;
- WMI repository querying;
- startup folder filesystem scanning;
- persistence rollback;
- final detection scoring;
- alerts;
- remediation;
- named-pipe IPC;
- UI streaming;
- real ETW callbacks.

## Task 1: Persistence Metadata Analyzer

**Files:**

- Create: `crates/engine-persistence/Cargo.toml`
- Create: `crates/engine-persistence/src/lib.rs`
- Create: `crates/engine-persistence/src/event.rs`
- Create: `crates/engine-persistence/src/signal.rs`
- Create: `crates/engine-persistence/src/analyzer.rs`
- Create: `crates/engine-persistence/tests/analyzer.rs`
- Modify: `Cargo.toml`

- [ ] **Step 1: Add workspace member and failing analyzer tests**

Add `"crates/engine-persistence"` to root `Cargo.toml` workspace members.

Create `crates/engine-persistence/Cargo.toml`:

```toml
[package]
name = "engine-persistence"
authors.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
rust-version.workspace = true
version.workspace = true

[dependencies]
shared-models = { path = "../shared-models" }
```

Create empty `crates/engine-persistence/src/lib.rs`.

Create `crates/engine-persistence/tests/analyzer.rs`:

```rust
use engine_persistence::{PersistenceAnalyzer, PersistenceKind, SignalSeverity};
use shared_models::{
    EventPriority, NormalizedTelemetryEvent, TelemetryAction, TelemetryMetadata,
    TelemetrySource, Timestamp,
};

fn timestamp(value: &str) -> Timestamp {
    Timestamp::parse_rfc3339(value).unwrap()
}

fn persistence_event(
    kind: &str,
    path: &str,
    value: &str,
    operation: &str,
) -> NormalizedTelemetryEvent {
    let mut event = NormalizedTelemetryEvent::new(
        TelemetrySource::WindowsEventLog,
        EventPriority::Medium,
        TelemetryAction::RegistryChanged,
        timestamp("2026-06-27T09:00:00Z"),
    );
    event.metadata = TelemetryMetadata::empty()
        .insert("persistence.kind", kind)
        .insert("persistence.path", path)
        .insert("persistence.value", value)
        .insert("persistence.operation", operation);
    event
}

#[test]
fn run_key_metadata_emits_registry_run_key_signal() {
    let mut analyzer = PersistenceAnalyzer::default();
    let report = analyzer.analyze(persistence_event(
        "registry_run_key",
        r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run",
        "Updater",
        "set_value",
    ));

    assert_eq!(report.stats.observed, 1);
    assert_eq!(report.stats.handled, 1);
    assert_eq!(report.stats.ignored, 0);
    assert_eq!(report.signals.len(), 1);
    assert_eq!(report.signals[0].name, "registry_run_key_persistence");
    assert_eq!(report.signals[0].severity, SignalSeverity::High);
    assert_eq!(report.signals[0].event.kind, PersistenceKind::RegistryRunKey);
}

#[test]
fn startup_folder_metadata_emits_startup_folder_signal() {
    let mut analyzer = PersistenceAnalyzer::default();
    let report = analyzer.analyze(persistence_event(
        "startup_folder",
        r"C:\Users\alice\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup\run.lnk",
        "run.lnk",
        "created",
    ));

    assert_eq!(report.signals.len(), 1);
    assert_eq!(report.signals[0].name, "startup_folder_persistence");
    assert_eq!(report.signals[0].severity, SignalSeverity::Medium);
    assert_eq!(report.signals[0].event.kind, PersistenceKind::StartupFolder);
}

#[test]
fn scheduled_task_metadata_emits_scheduled_task_signal() {
    let mut analyzer = PersistenceAnalyzer::default();
    let report = analyzer.analyze(persistence_event(
        "scheduled_task",
        r"C:\Windows\System32\Tasks\Updater",
        "Updater",
        "created",
    ));

    assert_eq!(report.signals.len(), 1);
    assert_eq!(report.signals[0].name, "scheduled_task_persistence");
    assert_eq!(report.signals[0].severity, SignalSeverity::Medium);
    assert_eq!(report.signals[0].event.kind, PersistenceKind::ScheduledTask);
}

#[test]
fn service_metadata_emits_service_signal() {
    let mut analyzer = PersistenceAnalyzer::default();
    let report = analyzer.analyze(persistence_event(
        "service",
        r"HKLM\System\CurrentControlSet\Services\Updater",
        "ImagePath",
        "set_value",
    ));

    assert_eq!(report.signals.len(), 1);
    assert_eq!(report.signals[0].name, "service_persistence");
    assert_eq!(report.signals[0].severity, SignalSeverity::High);
    assert_eq!(report.signals[0].event.kind, PersistenceKind::Service);
}

#[test]
fn wmi_metadata_emits_wmi_signal() {
    let mut analyzer = PersistenceAnalyzer::default();
    let report = analyzer.analyze(persistence_event(
        "wmi",
        r"ROOT\subscription:__EventFilter.Name='Updater'",
        "__EventFilter",
        "created",
    ));

    assert_eq!(report.signals.len(), 1);
    assert_eq!(report.signals[0].name, "wmi_persistence");
    assert_eq!(report.signals[0].severity, SignalSeverity::High);
    assert_eq!(report.signals[0].event.kind, PersistenceKind::WmiSubscription);
}

#[test]
fn irrelevant_telemetry_is_counted_as_ignored() {
    let mut analyzer = PersistenceAnalyzer::default();
    let event = NormalizedTelemetryEvent::new(
        TelemetrySource::InternalHealth,
        EventPriority::Low,
        TelemetryAction::ComponentHealthChanged,
        timestamp("2026-06-27T09:00:00Z"),
    );

    let report = analyzer.analyze(event);

    assert_eq!(report.stats.observed, 1);
    assert_eq!(report.stats.handled, 0);
    assert_eq!(report.stats.ignored, 1);
    assert!(report.signals.is_empty());
}
```

- [ ] **Step 2: Run analyzer tests to verify RED**

Run:

```powershell
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
cargo test -p engine-persistence --test analyzer
```

Expected: compile failure mentioning missing `PersistenceAnalyzer`, `PersistenceKind`, or `SignalSeverity`.

- [ ] **Step 3: Implement metadata parser, signals, and analyzer**

Replace `crates/engine-persistence/src/lib.rs`:

```rust
pub mod analyzer;
pub mod event;
pub mod signal;

pub use analyzer::{PersistenceAnalysisReport, PersistenceAnalysisStats, PersistenceAnalyzer};
pub use event::{PersistenceEvent, PersistenceKind};
pub use signal::{PersistenceSignal, SignalSeverity};
```

Create `crates/engine-persistence/src/event.rs`:

```rust
use shared_models::{NormalizedTelemetryEvent, TelemetryEventId, Timestamp};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersistenceKind {
    RegistryRunKey,
    StartupFolder,
    ScheduledTask,
    Service,
    WmiSubscription,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistenceEvent {
    pub kind: PersistenceKind,
    pub path: String,
    pub value: String,
    pub operation: String,
    pub image_path: Option<String>,
    pub command: Option<String>,
    pub user: Option<String>,
    pub supporting_event_id: TelemetryEventId,
    pub observed_at: Timestamp,
}

impl PersistenceEvent {
    pub fn from_telemetry(event: &NormalizedTelemetryEvent) -> Option<Self> {
        let path = event.metadata.get("persistence.path")?.to_string();
        let value = event
            .metadata
            .get("persistence.value")
            .unwrap_or_default()
            .to_string();
        let operation = event
            .metadata
            .get("persistence.operation")
            .unwrap_or_default()
            .to_string();
        let kind_hint = event.metadata.get("persistence.kind").unwrap_or_default();
        let kind = classify_kind(kind_hint, &path, &value);

        Some(Self {
            kind,
            path,
            value,
            operation,
            image_path: event.metadata.get("persistence.image_path").map(str::to_string),
            command: event.metadata.get("persistence.command").map(str::to_string),
            user: event.metadata.get("persistence.user").map(str::to_string),
            supporting_event_id: event.event_id.clone(),
            observed_at: event.timestamp.clone(),
        })
    }
}

fn classify_kind(kind_hint: &str, path: &str, value: &str) -> PersistenceKind {
    let kind_hint = normalize(kind_hint);
    let path = normalize(path);
    let value = normalize(value);
    let combined = format!("{kind_hint} {path} {value}");

    if combined.contains("runonce")
        || combined.contains("registry_run_key")
        || combined.contains(r"\software\microsoft\windows\currentversion\run")
    {
        PersistenceKind::RegistryRunKey
    } else if combined.contains("startup_folder")
        || combined.contains(r"\start menu\programs\startup\")
        || combined.contains(r"\startup\")
    {
        PersistenceKind::StartupFolder
    } else if combined.contains("scheduled_task")
        || combined.contains(r"\system32\tasks\")
        || combined.contains(r"\microsoft\windows\task scheduler\")
    {
        PersistenceKind::ScheduledTask
    } else if combined.contains("service")
        || combined.contains(r"\system\currentcontrolset\services\")
    {
        PersistenceKind::Service
    } else if combined.contains("wmi")
        || combined.contains("__eventfilter")
        || combined.contains("commandlineeventconsumer")
        || combined.contains("__filtertoconsumerbinding")
    {
        PersistenceKind::WmiSubscription
    } else {
        PersistenceKind::Unknown
    }
}

fn normalize(value: &str) -> String {
    value.replace('/', r"\").to_ascii_lowercase()
}
```

Create `crates/engine-persistence/src/signal.rs`:

```rust
use crate::event::{PersistenceEvent, PersistenceKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalSeverity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistenceSignal {
    pub name: String,
    pub description: String,
    pub severity: SignalSeverity,
    pub event: PersistenceEvent,
}

pub fn signal_for_event(event: &PersistenceEvent) -> Option<PersistenceSignal> {
    match event.kind {
        PersistenceKind::RegistryRunKey => Some(PersistenceSignal {
            name: "registry_run_key_persistence".to_string(),
            description: "Persistence metadata indicates Run or RunOnce key modification"
                .to_string(),
            severity: SignalSeverity::High,
            event: event.clone(),
        }),
        PersistenceKind::StartupFolder => Some(PersistenceSignal {
            name: "startup_folder_persistence".to_string(),
            description: "Persistence metadata indicates startup folder modification".to_string(),
            severity: SignalSeverity::Medium,
            event: event.clone(),
        }),
        PersistenceKind::ScheduledTask => Some(PersistenceSignal {
            name: "scheduled_task_persistence".to_string(),
            description: "Persistence metadata indicates scheduled task creation or modification"
                .to_string(),
            severity: SignalSeverity::Medium,
            event: event.clone(),
        }),
        PersistenceKind::Service => Some(PersistenceSignal {
            name: "service_persistence".to_string(),
            description: "Persistence metadata indicates service creation or service path change"
                .to_string(),
            severity: SignalSeverity::High,
            event: event.clone(),
        }),
        PersistenceKind::WmiSubscription => Some(PersistenceSignal {
            name: "wmi_persistence".to_string(),
            description: "Persistence metadata indicates WMI permanent event subscription"
                .to_string(),
            severity: SignalSeverity::High,
            event: event.clone(),
        }),
        PersistenceKind::Unknown => None,
    }
}
```

Create `crates/engine-persistence/src/analyzer.rs`:

```rust
use crate::event::PersistenceEvent;
use crate::signal::{PersistenceSignal, signal_for_event};
use shared_models::{ComponentHealth, HealthStatus, NormalizedTelemetryEvent, Timestamp};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PersistenceAnalysisStats {
    pub observed: u64,
    pub handled: u64,
    pub ignored: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistenceAnalysisReport {
    pub stats: PersistenceAnalysisStats,
    pub signals: Vec<PersistenceSignal>,
    pub component_health: ComponentHealth,
}

#[derive(Debug, Default)]
pub struct PersistenceAnalyzer {
    stats: PersistenceAnalysisStats,
}

impl PersistenceAnalyzer {
    pub fn analyze(&mut self, event: NormalizedTelemetryEvent) -> PersistenceAnalysisReport {
        self.stats.observed += 1;
        let mut signals = Vec::new();

        match PersistenceEvent::from_telemetry(&event) {
            Some(persistence_event) => {
                self.stats.handled += 1;
                if let Some(signal) = signal_for_event(&persistence_event) {
                    signals.push(signal);
                }
            }
            None => {
                self.stats.ignored += 1;
            }
        }

        PersistenceAnalysisReport {
            stats: self.stats.clone(),
            signals,
            component_health: ComponentHealth {
                component: "engine-persistence".to_string(),
                status: HealthStatus::Healthy,
                observed_at: Timestamp::now(),
                queue: None,
            },
        }
    }
}
```

- [ ] **Step 4: Run persistence engine tests to verify GREEN**

Run:

```powershell
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
cargo test -p engine-persistence
```

Expected: analyzer tests pass.

- [ ] **Step 5: Commit persistence analyzer**

Run:

```powershell
git add Cargo.toml Cargo.lock crates/engine-persistence
git commit -m "feat: add persistence signal analyzer"
```

Expected output contains `feat: add persistence signal analyzer`.

## Task 2: Agent Synthetic Persistence Analysis Dry Run

**Files:**

- Create: `crates/sentra-agent/src/persistence_dry_run.rs`
- Create: `crates/sentra-agent/tests/persistence_dry_run.rs`
- Modify: `crates/sentra-agent/Cargo.toml`
- Modify: `crates/sentra-agent/src/lib.rs`
- Modify: `crates/sentra-agent/src/main.rs`

- [ ] **Step 1: Write failing agent dry-run test**

Add `engine-persistence = { path = "../engine-persistence" }` to `crates/sentra-agent/Cargo.toml`.

Create `crates/sentra-agent/tests/persistence_dry_run.rs`:

```rust
use sentra_agent::persistence_dry_run::run_synthetic_persistence_analysis_dry_run;

#[test]
fn synthetic_persistence_analysis_reports_signals() {
    let report = run_synthetic_persistence_analysis_dry_run();

    assert_eq!(report.stats.observed, 2);
    assert_eq!(report.stats.handled, 2);
    assert_eq!(report.stats.ignored, 0);
    assert_eq!(report.signals.len(), 2);
    assert!(
        report
            .signals
            .iter()
            .any(|signal| signal.name == "registry_run_key_persistence")
    );
    assert!(
        report
            .signals
            .iter()
            .any(|signal| signal.name == "service_persistence")
    );
}
```

- [ ] **Step 2: Run dry-run test to verify RED**

Run:

```powershell
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
cargo test -p sentra-agent --test persistence_dry_run
```

Expected: compile failure mentioning missing `sentra_agent::persistence_dry_run`.

- [ ] **Step 3: Implement synthetic persistence dry run**

Modify `crates/sentra-agent/src/lib.rs`:

```rust
pub mod config;
pub mod dry_run;
pub mod logging;
pub mod persistence_dry_run;
pub mod process_dry_run;
```

Create `crates/sentra-agent/src/persistence_dry_run.rs`:

```rust
use engine_persistence::{PersistenceAnalysisReport, PersistenceAnalyzer};
use shared_models::{
    EventPriority, NormalizedTelemetryEvent, TelemetryAction, TelemetryMetadata,
    TelemetrySource, Timestamp,
};

pub fn run_synthetic_persistence_analysis_dry_run() -> PersistenceAnalysisReport {
    let mut analyzer = PersistenceAnalyzer::default();

    analyzer.analyze(persistence_event(
        "registry_run_key",
        r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run",
        "Updater",
        "set_value",
        "2026-06-27T09:02:00Z",
    ));

    analyzer.analyze(persistence_event(
        "service",
        r"HKLM\System\CurrentControlSet\Services\Updater",
        "ImagePath",
        "set_value",
        "2026-06-27T09:03:00Z",
    ))
}

fn persistence_event(
    kind: &str,
    path: &str,
    value: &str,
    operation: &str,
    observed_at: &str,
) -> NormalizedTelemetryEvent {
    let mut event = NormalizedTelemetryEvent::new(
        TelemetrySource::WindowsEventLog,
        EventPriority::Medium,
        TelemetryAction::RegistryChanged,
        Timestamp::parse_rfc3339(observed_at).unwrap(),
    );
    event.metadata = TelemetryMetadata::empty()
        .insert("persistence.kind", kind)
        .insert("persistence.path", path)
        .insert("persistence.value", value)
        .insert("persistence.operation", operation);
    event
}
```

Modify `crates/sentra-agent/src/main.rs`:

```rust
use sentra_agent::config::AgentConfig;
use sentra_agent::dry_run::run_synthetic_etw_dry_run;
use sentra_agent::logging::init_logging;
use sentra_agent::persistence_dry_run::run_synthetic_persistence_analysis_dry_run;
use sentra_agent::process_dry_run::run_synthetic_process_analysis_dry_run;
use tracing::info;

fn main() {
    let config = AgentConfig::default();
    init_logging(&config.logging.level);

    if let Err(error) = config.validate() {
        eprintln!("SentraEDR agent configuration error: {error}");
        std::process::exit(2);
    }

    let etw_report = run_synthetic_etw_dry_run();
    let process_report = run_synthetic_process_analysis_dry_run();
    let persistence_report = run_synthetic_persistence_analysis_dry_run();

    info!(
        mode = ?config.mode,
        telemetry_capacity = config.queue.telemetry_capacity,
        detection_capacity = config.queue.detection_capacity,
        etw_received = etw_report.stats.received,
        etw_normalized = etw_report.stats.normalized,
        etw_dropped = etw_report.stats.dropped,
        process_observed = process_report.stats.observed,
        process_started = process_report.stats.started,
        process_signals = process_report.signals.len(),
        process_tracked = process_report.tracked_processes,
        persistence_observed = persistence_report.stats.observed,
        persistence_handled = persistence_report.stats.handled,
        persistence_signals = persistence_report.signals.len(),
        "SentraEDR agent foundation initialized in observe-only mode"
    );
}
```

- [ ] **Step 4: Run agent dry-run tests and binary**

Run:

```powershell
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
cargo test -p sentra-agent --test persistence_dry_run
cargo run -p sentra-agent
```

Expected: test passes and the binary log includes `persistence_observed=2`, `persistence_handled=2`, and `persistence_signals=2`.

- [ ] **Step 5: Commit agent persistence dry run**

Run:

```powershell
git add Cargo.toml Cargo.lock crates/sentra-agent
git commit -m "feat: add agent persistence analysis dry run"
```

Expected output contains `feat: add agent persistence analysis dry run`.

## Task 3: Architecture Validation And Documentation

**Files:**

- Modify: `tools/validate-architecture.ps1`
- Modify: `ARCHITECTURE.md`
- Modify: `PERFORMANCE_NOTES.md`
- Modify: `TASKS.md`
- Create: `PHASE_REPORTS/phase-4.md`
- Create: `TEST_RESULTS/phase-4.md`

- [ ] **Step 1: Update architecture validation**

Add rules that prevent `engine-persistence` from depending on agent, UI, ETW, process, detection, remediation, and peer engines:

```powershell
@{ Crate = "engine-persistence"; Pattern = 'path\s*=\s*"\.\./(sentra-agent|sentra-ui)"'; Message = "engine-persistence must not depend on agent or UI crates" },
@{ Crate = "engine-persistence"; Pattern = 'path\s*=\s*"\.\./engine-'; Message = "engine-persistence must not depend on peer engine crates" }
```

- [ ] **Step 2: Run architecture validation**

Run:

```powershell
powershell -ExecutionPolicy Bypass -File tools\validate-architecture.ps1
```

Expected: `Architecture dependency validation passed.`

- [ ] **Step 3: Update documentation**

Update docs to state:

- Phase 4 added `engine-persistence`.
- Phase 4 parses persistence metadata from normalized telemetry.
- Phase 4 emits observe-only preliminary persistence signals.
- Phase 4 does not implement registry writes, task/service/WMI APIs, final detection scoring, alerts, remediation, or UI.

Create `PHASE_REPORTS/phase-4.md` with active roles:

- Windows Persistence Engineer.
- Rust Systems Engineer.
- Detection Strategy Engineer.
- Performance Engineer.
- QA / Validation Engineer.
- Documentation Engineer.

Create `TEST_RESULTS/phase-4.md` with exact verification commands and results after final verification.

- [ ] **Step 4: Commit documentation**

Run:

```powershell
git add tools/validate-architecture.ps1 ARCHITECTURE.md PERFORMANCE_NOTES.md TASKS.md PHASE_REPORTS/phase-4.md TEST_RESULTS/phase-4.md
git commit -m "docs: record phase 4 validation"
```

Expected output contains `docs: record phase 4 validation`.

## Final Verification

- [ ] **Step 1: Run all checks**

Run:

```powershell
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
powershell -ExecutionPolicy Bypass -File tools\validate-architecture.ps1
cargo run -p sentra-agent
```

Expected:

- format check exits 0;
- clippy exits 0;
- all workspace tests pass;
- architecture validation passes;
- agent dry run logs ETW, process, and persistence analysis counts.

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

## Self-Review

Spec coverage:

- `engine-persistence` crate: Task 1.
- Metadata parser and persistence event model: Task 1.
- Run key signal: Task 1.
- Startup folder signal: Task 1.
- Scheduled task signal: Task 1.
- Service signal: Task 1.
- WMI signal: Task 1.
- Irrelevant telemetry ignored without panic: Task 1.
- Agent synthetic dry run: Task 2.
- Architecture validation and docs: Task 3.

Placeholder scan:

- The plan contains concrete file paths, commands, expected outputs, and code snippets for every behavior-changing step.

Type consistency:

- `PersistenceAnalyzer::analyze` consumes `NormalizedTelemetryEvent` and returns `PersistenceAnalysisReport`.
- `PersistenceEvent::from_telemetry` reads `TelemetryMetadata` keys.
- `PersistenceSignal` uses `SignalSeverity`, `PersistenceEvent`, and signal names from the design.
- Agent dry run calls `run_synthetic_persistence_analysis_dry_run`.
