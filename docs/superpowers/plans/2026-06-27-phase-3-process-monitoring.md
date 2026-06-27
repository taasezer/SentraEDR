# Phase 3 Process Monitoring Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add `engine-process`, a lightweight observe-only process monitoring engine that consumes normalized process telemetry, maintains lifecycle state, and emits preliminary process behavior signals.

**Architecture:** `engine-process` consumes `shared-models::NormalizedTelemetryEvent` only, so it stays decoupled from ETW-specific records. It owns process state, deterministic signal matching, and analysis reports; `sentra-agent` only runs a synthetic dry run and logs counts.

**Tech Stack:** Rust 1.85+ edition 2024, existing workspace crates, `shared-models`, `thiserror` only if required, `stable-x86_64-pc-windows-gnu` for local validation.

---

## Source Inputs

- `docs/superpowers/specs/2026-06-27-phase-3-process-monitoring-design.md`
- `crates/shared-models/src/telemetry.rs`
- `crates/shared-models/src/process.rs`
- `crates/shared-models/src/health.rs`
- `crates/engine-etw/src/normalize.rs`
- `crates/sentra-agent/src/dry_run.rs`
- `tools/validate-architecture.ps1`

## File Structure

Create:

- `crates/engine-process/Cargo.toml`: process engine crate manifest.
- `crates/engine-process/src/lib.rs`: public exports.
- `crates/engine-process/src/state.rs`: process lifecycle state table.
- `crates/engine-process/src/signal.rs`: process signal types and deterministic matchers.
- `crates/engine-process/src/analyzer.rs`: event-to-state/report orchestration.
- `crates/engine-process/tests/state.rs`: state table tests.
- `crates/engine-process/tests/signals.rs`: behavior signal tests.
- `crates/sentra-agent/tests/process_dry_run.rs`: agent synthetic process analysis test.
- `PHASE_REPORTS/phase-3.md`: Phase 3 report.
- `TEST_RESULTS/phase-3.md`: Phase 3 verification record.

Modify:

- `Cargo.toml`: add `crates/engine-process`.
- `crates/sentra-agent/Cargo.toml`: depend on `engine-process`.
- `crates/sentra-agent/src/lib.rs`: export `process_dry_run`.
- `crates/sentra-agent/src/main.rs`: log process analysis dry-run counts.
- `crates/sentra-agent/src/process_dry_run.rs`: synthetic process analysis dry run.
- `tools/validate-architecture.ps1`: add `engine-process` boundary checks.
- `ARCHITECTURE.md`: mark Phase 3 process engine boundary.
- `PERFORMANCE_NOTES.md`: record synthetic state/signal validation.
- `TASKS.md`: mark Phase 3 complete after verification.

Do not create:

- real Windows process enumeration;
- signature or reputation checks;
- detection engine scoring;
- alerts;
- remediation;
- named-pipe IPC;
- UI streaming;
- real ETW callbacks.

## Task 1: Process State Table

**Files:**

- Create: `crates/engine-process/Cargo.toml`
- Create: `crates/engine-process/src/lib.rs`
- Create: `crates/engine-process/src/state.rs`
- Create: `crates/engine-process/tests/state.rs`
- Modify: `Cargo.toml`

- [ ] **Step 1: Add workspace member and failing state tests**

Add `"crates/engine-process"` to root `Cargo.toml` workspace members.

Create `crates/engine-process/Cargo.toml`:

```toml
[package]
name = "engine-process"
authors.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
rust-version.workspace = true
version.workspace = true

[dependencies]
shared-models = { path = "../shared-models" }
```

Create empty `crates/engine-process/src/lib.rs`.

Create `crates/engine-process/tests/state.rs`:

```rust
use engine_process::{ProcessLifecycleStatus, ProcessStateTable, ProcessStateUpdate};
use shared_models::{
    CommandLine, EventPriority, ImagePath, NormalizedTelemetryEvent, ProcessIdentity,
    TelemetryAction, TelemetrySource, Timestamp,
};

fn timestamp(value: &str) -> Timestamp {
    Timestamp::parse_rfc3339(value).unwrap()
}

fn process_event(
    action: TelemetryAction,
    process_id: u32,
    parent_process_id: Option<u32>,
    image_path: &str,
    command_line: &str,
    observed_at: &str,
) -> NormalizedTelemetryEvent {
    let mut process = ProcessIdentity::new(process_id)
        .with_image_path(ImagePath::new(image_path))
        .with_command_line(CommandLine::new(command_line));
    process.parent_process_id = parent_process_id;

    NormalizedTelemetryEvent::new(
        TelemetrySource::Etw,
        EventPriority::Medium,
        action,
        timestamp(observed_at),
    )
    .with_process(process)
    .with_confidence_hint(100)
}

#[test]
fn process_start_inserts_running_snapshot() {
    let mut table = ProcessStateTable::default();
    let event = process_event(
        TelemetryAction::ProcessStarted,
        4242,
        Some(1000),
        r"C:\Windows\System32\cmd.exe",
        "cmd.exe /c whoami",
        "2026-06-27T09:00:00Z",
    );

    let update = table.apply_event(&event);

    assert!(matches!(update, ProcessStateUpdate::Started(_)));
    let snapshot = table.get(4242).unwrap();
    assert_eq!(snapshot.process.process_id, 4242);
    assert_eq!(snapshot.process.parent_process_id, Some(1000));
    assert_eq!(snapshot.status, ProcessLifecycleStatus::Running);
    assert_eq!(snapshot.first_observed, timestamp("2026-06-27T09:00:00Z"));
    assert_eq!(snapshot.last_observed, timestamp("2026-06-27T09:00:00Z"));
    assert_eq!(table.len(), 1);
}

#[test]
fn process_exit_marks_existing_process_as_exited() {
    let mut table = ProcessStateTable::default();
    let start = process_event(
        TelemetryAction::ProcessStarted,
        4242,
        Some(1000),
        r"C:\Windows\System32\cmd.exe",
        "cmd.exe",
        "2026-06-27T09:00:00Z",
    );
    let exit = process_event(
        TelemetryAction::ProcessExited,
        4242,
        Some(1000),
        r"C:\Windows\System32\cmd.exe",
        "cmd.exe",
        "2026-06-27T09:01:00Z",
    );

    table.apply_event(&start);
    let update = table.apply_event(&exit);

    assert!(matches!(update, ProcessStateUpdate::Exited(_)));
    let snapshot = table.get(4242).unwrap();
    assert_eq!(snapshot.status, ProcessLifecycleStatus::Exited);
    assert_eq!(snapshot.first_observed, timestamp("2026-06-27T09:00:00Z"));
    assert_eq!(snapshot.last_observed, timestamp("2026-06-27T09:01:00Z"));
    assert_eq!(table.len(), 1);
}

#[test]
fn irrelevant_telemetry_is_ignored_without_state_change() {
    let mut table = ProcessStateTable::default();
    let event = NormalizedTelemetryEvent::new(
        TelemetrySource::InternalHealth,
        EventPriority::Low,
        TelemetryAction::ComponentHealthChanged,
        timestamp("2026-06-27T09:00:00Z"),
    );

    let update = table.apply_event(&event);

    assert_eq!(update, ProcessStateUpdate::Ignored);
    assert_eq!(table.len(), 0);
}
```

- [ ] **Step 2: Run state tests to verify RED**

Run:

```powershell
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
cargo test -p engine-process --test state
```

Expected: compile failure mentioning missing `ProcessLifecycleStatus`, `ProcessStateTable`, or `ProcessStateUpdate`.

- [ ] **Step 3: Implement state table**

Replace `crates/engine-process/src/lib.rs`:

```rust
pub mod state;

pub use state::{
    ProcessLifecycleStatus, ProcessSnapshot, ProcessStateTable, ProcessStateUpdate,
};
```

Create `crates/engine-process/src/state.rs`:

```rust
use shared_models::{NormalizedTelemetryEvent, ProcessIdentity, TelemetryAction, Timestamp};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessLifecycleStatus {
    Running,
    Exited,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessSnapshot {
    pub process: ProcessIdentity,
    pub first_observed: Timestamp,
    pub last_observed: Timestamp,
    pub status: ProcessLifecycleStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessStateUpdate {
    Started(ProcessSnapshot),
    Exited(ProcessSnapshot),
    Ignored,
}

#[derive(Debug, Default)]
pub struct ProcessStateTable {
    processes: BTreeMap<u32, ProcessSnapshot>,
}

impl ProcessStateTable {
    pub fn apply_event(&mut self, event: &NormalizedTelemetryEvent) -> ProcessStateUpdate {
        let Some(process) = event.process.clone() else {
            return ProcessStateUpdate::Ignored;
        };

        match event.action {
            TelemetryAction::ProcessStarted => self.apply_start(process, event.timestamp),
            TelemetryAction::ProcessExited => self.apply_exit(process, event.timestamp),
            _ => ProcessStateUpdate::Ignored,
        }
    }

    pub fn get(&self, process_id: u32) -> Option<&ProcessSnapshot> {
        self.processes.get(&process_id)
    }

    pub fn len(&self) -> usize {
        self.processes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.processes.is_empty()
    }

    fn apply_start(
        &mut self,
        process: ProcessIdentity,
        observed_at: Timestamp,
    ) -> ProcessStateUpdate {
        let snapshot = ProcessSnapshot {
            process,
            first_observed: observed_at,
            last_observed: observed_at,
            status: ProcessLifecycleStatus::Running,
        };
        self.processes
            .insert(snapshot.process.process_id, snapshot.clone());
        ProcessStateUpdate::Started(snapshot)
    }

    fn apply_exit(
        &mut self,
        process: ProcessIdentity,
        observed_at: Timestamp,
    ) -> ProcessStateUpdate {
        let process_id = process.process_id;
        let snapshot = match self.processes.get_mut(&process_id) {
            Some(existing) => {
                existing.last_observed = observed_at;
                existing.status = ProcessLifecycleStatus::Exited;
                existing.clone()
            }
            None => {
                let snapshot = ProcessSnapshot {
                    process,
                    first_observed: observed_at,
                    last_observed: observed_at,
                    status: ProcessLifecycleStatus::Exited,
                };
                self.processes.insert(process_id, snapshot.clone());
                snapshot
            }
        };

        ProcessStateUpdate::Exited(snapshot)
    }
}
```

- [ ] **Step 4: Run state tests to verify GREEN**

Run:

```powershell
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
cargo test -p engine-process --test state
```

Expected: `3 passed`.

- [ ] **Step 5: Commit state table**

Run:

```powershell
git add Cargo.toml Cargo.lock crates/engine-process
git commit -m "feat: add process state table"
```

Expected output contains `feat: add process state table`.

## Task 2: Process Signal Analyzer

**Files:**

- Create: `crates/engine-process/src/signal.rs`
- Create: `crates/engine-process/src/analyzer.rs`
- Create: `crates/engine-process/tests/signals.rs`
- Modify: `crates/engine-process/src/lib.rs`

- [ ] **Step 1: Write failing signal tests**

Create `crates/engine-process/tests/signals.rs`:

```rust
use engine_process::{ProcessAnalyzer, SignalSeverity};
use shared_models::{
    CommandLine, EventPriority, ImagePath, NormalizedTelemetryEvent, ProcessIdentity,
    TelemetryAction, TelemetrySource, Timestamp,
};

fn timestamp(value: &str) -> Timestamp {
    Timestamp::parse_rfc3339(value).unwrap()
}

fn start_event(
    process_id: u32,
    parent_process_id: Option<u32>,
    image_path: &str,
    command_line: &str,
) -> NormalizedTelemetryEvent {
    let mut process = ProcessIdentity::new(process_id)
        .with_image_path(ImagePath::new(image_path))
        .with_command_line(CommandLine::new(command_line));
    process.parent_process_id = parent_process_id;

    NormalizedTelemetryEvent::new(
        TelemetrySource::Etw,
        EventPriority::Medium,
        TelemetryAction::ProcessStarted,
        timestamp("2026-06-27T09:00:00Z"),
    )
    .with_process(process)
    .with_confidence_hint(100)
}

#[test]
fn office_to_powershell_emits_suspicious_parent_child_signal() {
    let mut analyzer = ProcessAnalyzer::default();

    analyzer.analyze(start_event(
        1000,
        None,
        r"C:\Program Files\Microsoft Office\root\Office16\WINWORD.EXE",
        "WINWORD.EXE report.docx",
    ));
    let report = analyzer.analyze(start_event(
        4242,
        Some(1000),
        r"C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe",
        "powershell.exe -NoProfile",
    ));

    assert_eq!(report.stats.observed, 2);
    assert_eq!(report.stats.started, 2);
    assert_eq!(report.stats.ignored, 0);
    assert_eq!(report.tracked_processes, 2);
    assert_eq!(report.signals.len(), 1);
    assert_eq!(report.signals[0].name, "suspicious_parent_child");
    assert_eq!(report.signals[0].severity, SignalSeverity::High);
    assert_eq!(report.signals[0].parent.as_ref().unwrap().process.process_id, 1000);
}

#[test]
fn powershell_encoded_command_emits_signal() {
    let mut analyzer = ProcessAnalyzer::default();

    let report = analyzer.analyze(start_event(
        4242,
        None,
        r"C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe",
        "powershell.exe -EncodedCommand SQBFAFgA",
    ));

    assert_eq!(report.signals.len(), 1);
    assert_eq!(report.signals[0].name, "powershell_encoded_command");
    assert_eq!(report.signals[0].severity, SignalSeverity::Medium);
}

#[test]
fn user_writable_execution_path_emits_signal() {
    let mut analyzer = ProcessAnalyzer::default();

    let report = analyzer.analyze(start_event(
        4242,
        None,
        r"C:\Users\alice\AppData\Local\Temp\payload.exe",
        r"C:\Users\alice\AppData\Local\Temp\payload.exe",
    ));

    assert_eq!(report.signals.len(), 1);
    assert_eq!(report.signals[0].name, "user_writable_execution_path");
    assert_eq!(report.signals[0].severity, SignalSeverity::Medium);
}

#[test]
fn non_process_event_is_counted_as_ignored() {
    let mut analyzer = ProcessAnalyzer::default();
    let event = NormalizedTelemetryEvent::new(
        TelemetrySource::InternalHealth,
        EventPriority::Low,
        TelemetryAction::ComponentHealthChanged,
        timestamp("2026-06-27T09:00:00Z"),
    );

    let report = analyzer.analyze(event);

    assert_eq!(report.stats.observed, 1);
    assert_eq!(report.stats.started, 0);
    assert_eq!(report.stats.exited, 0);
    assert_eq!(report.stats.ignored, 1);
    assert!(report.signals.is_empty());
}
```

- [ ] **Step 2: Run signal tests to verify RED**

Run:

```powershell
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
cargo test -p engine-process --test signals
```

Expected: compile failure mentioning missing `ProcessAnalyzer`, `SignalSeverity`, or `signals`.

- [ ] **Step 3: Implement signals and analyzer**

Update `crates/engine-process/src/lib.rs`:

```rust
pub mod analyzer;
pub mod signal;
pub mod state;

pub use analyzer::{ProcessAnalysisReport, ProcessAnalysisStats, ProcessAnalyzer};
pub use signal::{ProcessSignal, SignalSeverity};
pub use state::{
    ProcessLifecycleStatus, ProcessSnapshot, ProcessStateTable, ProcessStateUpdate,
};
```

Create `crates/engine-process/src/signal.rs`:

```rust
use crate::state::ProcessSnapshot;
use shared_models::TelemetryEventId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalSeverity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessSignal {
    pub name: String,
    pub description: String,
    pub severity: SignalSeverity,
    pub process: ProcessSnapshot,
    pub parent: Option<ProcessSnapshot>,
    pub supporting_event_id: TelemetryEventId,
}

pub fn signals_for_start(
    process: &ProcessSnapshot,
    parent: Option<&ProcessSnapshot>,
    supporting_event_id: TelemetryEventId,
) -> Vec<ProcessSignal> {
    let mut signals = Vec::new();

    if let Some(parent) = parent {
        if is_suspicious_parent_child(parent, process) {
            signals.push(ProcessSignal {
                name: "suspicious_parent_child".to_string(),
                description: "Process lineage matched a suspicious parent-child pair".to_string(),
                severity: SignalSeverity::High,
                process: process.clone(),
                parent: Some(parent.clone()),
                supporting_event_id: supporting_event_id.clone(),
            });
        }
    }

    if has_powershell_encoded_command(process) {
        signals.push(ProcessSignal {
            name: "powershell_encoded_command".to_string(),
            description: "PowerShell command line contains an encoded command flag".to_string(),
            severity: SignalSeverity::Medium,
            process: process.clone(),
            parent: parent.cloned(),
            supporting_event_id: supporting_event_id.clone(),
        });
    }

    if runs_from_user_writable_path(process) {
        signals.push(ProcessSignal {
            name: "user_writable_execution_path".to_string(),
            description: "Process image path appears to be under a user-writable location".to_string(),
            severity: SignalSeverity::Medium,
            process: process.clone(),
            parent: parent.cloned(),
            supporting_event_id,
        });
    }

    signals
}

fn is_suspicious_parent_child(parent: &ProcessSnapshot, child: &ProcessSnapshot) -> bool {
    let Some(parent_name) = executable_name(parent) else {
        return false;
    };
    let Some(child_name) = executable_name(child) else {
        return false;
    };

    matches!(
        (parent_name.as_str(), child_name.as_str()),
        ("winword.exe", "powershell.exe")
            | ("excel.exe", "powershell.exe")
            | ("powerpnt.exe", "powershell.exe")
            | ("chrome.exe", "powershell.exe")
            | ("msedge.exe", "powershell.exe")
            | ("firefox.exe", "powershell.exe")
            | ("winrar.exe", "powershell.exe")
            | ("7z.exe", "powershell.exe")
    )
}

fn has_powershell_encoded_command(process: &ProcessSnapshot) -> bool {
    let Some(name) = executable_name(process) else {
        return false;
    };
    if name != "powershell.exe" && name != "pwsh.exe" {
        return false;
    }

    let Some(command_line) = process.process.command_line.as_ref() else {
        return false;
    };
    let command_line = command_line.as_str().to_ascii_lowercase();

    command_line.contains("-enc")
        || command_line.contains("/enc")
        || command_line.contains("-encodedcommand")
        || command_line.contains("/encodedcommand")
}

fn runs_from_user_writable_path(process: &ProcessSnapshot) -> bool {
    let Some(image_path) = process.process.image_path.as_ref() else {
        return false;
    };
    let image_path = image_path.as_str().replace('/', r"\").to_ascii_lowercase();

    image_path.contains(r"\appdata\local\temp\")
        || image_path.contains(r"\appdata\roaming\")
        || image_path.contains(r"\downloads\")
        || image_path.contains(r"\temp\")
        || image_path.contains(r"\users\")
}

fn executable_name(process: &ProcessSnapshot) -> Option<String> {
    let image_path = process.process.image_path.as_ref()?.as_str();
    image_path
        .rsplit(['\\', '/'])
        .next()
        .map(|name| name.to_ascii_lowercase())
}
```

Create `crates/engine-process/src/analyzer.rs`:

```rust
use crate::signal::{signals_for_start, ProcessSignal};
use crate::state::{ProcessStateTable, ProcessStateUpdate};
use shared_models::{ComponentHealth, HealthStatus, NormalizedTelemetryEvent, Timestamp};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProcessAnalysisStats {
    pub observed: u64,
    pub started: u64,
    pub exited: u64,
    pub ignored: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessAnalysisReport {
    pub stats: ProcessAnalysisStats,
    pub tracked_processes: usize,
    pub signals: Vec<ProcessSignal>,
    pub component_health: ComponentHealth,
}

#[derive(Debug, Default)]
pub struct ProcessAnalyzer {
    state: ProcessStateTable,
    stats: ProcessAnalysisStats,
}

impl ProcessAnalyzer {
    pub fn analyze(&mut self, event: NormalizedTelemetryEvent) -> ProcessAnalysisReport {
        self.stats.observed += 1;
        let mut signals = Vec::new();

        match self.state.apply_event(&event) {
            ProcessStateUpdate::Started(snapshot) => {
                self.stats.started += 1;
                let parent = snapshot
                    .process
                    .parent_process_id
                    .and_then(|parent_id| self.state.get(parent_id));
                signals = signals_for_start(&snapshot, parent, event.event_id);
            }
            ProcessStateUpdate::Exited(_) => {
                self.stats.exited += 1;
            }
            ProcessStateUpdate::Ignored => {
                self.stats.ignored += 1;
            }
        }

        ProcessAnalysisReport {
            stats: self.stats.clone(),
            tracked_processes: self.state.len(),
            signals,
            component_health: ComponentHealth {
                component: "engine-process".to_string(),
                status: HealthStatus::Healthy,
                observed_at: Timestamp::now(),
                queue: None,
            },
        }
    }
}
```

- [ ] **Step 4: Run process engine tests to verify GREEN**

Run:

```powershell
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
cargo test -p engine-process
```

Expected: state and signal tests pass.

- [ ] **Step 5: Commit signal analyzer**

Run:

```powershell
git add crates/engine-process
git commit -m "feat: add process signal analyzer"
```

Expected output contains `feat: add process signal analyzer`.

## Task 3: Agent Synthetic Process Analysis Dry Run

**Files:**

- Create: `crates/sentra-agent/src/process_dry_run.rs`
- Create: `crates/sentra-agent/tests/process_dry_run.rs`
- Modify: `crates/sentra-agent/Cargo.toml`
- Modify: `crates/sentra-agent/src/lib.rs`
- Modify: `crates/sentra-agent/src/main.rs`

- [ ] **Step 1: Write failing agent dry-run test**

Add `engine-process = { path = "../engine-process" }` to `crates/sentra-agent/Cargo.toml`.

Create `crates/sentra-agent/tests/process_dry_run.rs`:

```rust
use sentra_agent::process_dry_run::run_synthetic_process_analysis_dry_run;

#[test]
fn synthetic_process_analysis_reports_signals() {
    let report = run_synthetic_process_analysis_dry_run();

    assert_eq!(report.stats.observed, 2);
    assert_eq!(report.stats.started, 2);
    assert_eq!(report.stats.exited, 0);
    assert_eq!(report.stats.ignored, 0);
    assert_eq!(report.tracked_processes, 2);
    assert_eq!(report.signals.len(), 2);
    assert!(
        report
            .signals
            .iter()
            .any(|signal| signal.name == "suspicious_parent_child")
    );
    assert!(
        report
            .signals
            .iter()
            .any(|signal| signal.name == "powershell_encoded_command")
    );
}
```

- [ ] **Step 2: Run dry-run test to verify RED**

Run:

```powershell
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
cargo test -p sentra-agent --test process_dry_run
```

Expected: compile failure mentioning missing `sentra_agent::process_dry_run`.

- [ ] **Step 3: Implement synthetic process dry run**

Modify `crates/sentra-agent/src/lib.rs`:

```rust
pub mod config;
pub mod dry_run;
pub mod logging;
pub mod process_dry_run;
```

Create `crates/sentra-agent/src/process_dry_run.rs`:

```rust
use engine_process::{ProcessAnalysisReport, ProcessAnalyzer};
use shared_models::{
    CommandLine, EventPriority, ImagePath, NormalizedTelemetryEvent, ProcessIdentity,
    TelemetryAction, TelemetrySource, Timestamp,
};

pub fn run_synthetic_process_analysis_dry_run() -> ProcessAnalysisReport {
    let mut analyzer = ProcessAnalyzer::default();

    analyzer.analyze(process_start(
        1000,
        None,
        r"C:\Program Files\Microsoft Office\root\Office16\WINWORD.EXE",
        "WINWORD.EXE report.docx",
        "2026-06-27T09:00:00Z",
    ));

    analyzer.analyze(process_start(
        4242,
        Some(1000),
        r"C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe",
        "powershell.exe -EncodedCommand SQBFAFgA",
        "2026-06-27T09:00:05Z",
    ))
}

fn process_start(
    process_id: u32,
    parent_process_id: Option<u32>,
    image_path: &str,
    command_line: &str,
    observed_at: &str,
) -> NormalizedTelemetryEvent {
    let mut process = ProcessIdentity::new(process_id)
        .with_image_path(ImagePath::new(image_path))
        .with_command_line(CommandLine::new(command_line));
    process.parent_process_id = parent_process_id;

    NormalizedTelemetryEvent::new(
        TelemetrySource::Etw,
        EventPriority::Medium,
        TelemetryAction::ProcessStarted,
        Timestamp::parse_rfc3339(observed_at).unwrap(),
    )
    .with_process(process)
    .with_confidence_hint(100)
}
```

Modify `crates/sentra-agent/src/main.rs`:

```rust
use sentra_agent::config::AgentConfig;
use sentra_agent::dry_run::run_synthetic_etw_dry_run;
use sentra_agent::logging::init_logging;
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
        "SentraEDR agent foundation initialized in observe-only mode"
    );
}
```

- [ ] **Step 4: Run agent dry-run tests and binary**

Run:

```powershell
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
cargo test -p sentra-agent --test process_dry_run
cargo run -p sentra-agent
```

Expected: test passes and the binary log includes `process_observed=2`, `process_signals=2`, and `process_tracked=2`.

- [ ] **Step 5: Commit agent process dry run**

Run:

```powershell
git add Cargo.toml Cargo.lock crates/sentra-agent
git commit -m "feat: add agent process analysis dry run"
```

Expected output contains `feat: add agent process analysis dry run`.

## Task 4: Architecture Validation And Documentation

**Files:**

- Modify: `tools/validate-architecture.ps1`
- Modify: `ARCHITECTURE.md`
- Modify: `PERFORMANCE_NOTES.md`
- Modify: `TASKS.md`
- Create: `PHASE_REPORTS/phase-3.md`
- Create: `TEST_RESULTS/phase-3.md`

- [ ] **Step 1: Update architecture validation**

Add rules that prevent `engine-process` from depending on agent, UI, ETW-specific code, detection, remediation, and peer engines:

```powershell
@{ Crate = "engine-process"; Pattern = 'path\s*=\s*"\.\./(sentra-agent|sentra-ui)"'; Message = "engine-process must not depend on agent or UI crates" },
@{ Crate = "engine-process"; Pattern = 'path\s*=\s*"\.\./engine-'; Message = "engine-process must not depend on peer engine crates" }
```

- [ ] **Step 2: Run architecture validation**

Run:

```powershell
powershell -ExecutionPolicy Bypass -File tools\validate-architecture.ps1
```

Expected: `Architecture dependency validation passed.`

- [ ] **Step 3: Update documentation**

Update docs to state:

- Phase 3 added `engine-process`.
- Phase 3 tracks process lifecycle state from normalized telemetry.
- Phase 3 emits observe-only preliminary process signals.
- Phase 3 does not implement final detection scoring, alerts, remediation, real process enumeration, or UI.

Create `PHASE_REPORTS/phase-3.md` with active roles:

- Windows Process Internals Engineer.
- Rust Systems Engineer.
- Detection Strategy Engineer.
- Performance Engineer.
- QA / Validation Engineer.
- Documentation Engineer.

Create `TEST_RESULTS/phase-3.md` with exact verification commands and results after final verification.

- [ ] **Step 4: Commit documentation**

Run:

```powershell
git add tools/validate-architecture.ps1 ARCHITECTURE.md PERFORMANCE_NOTES.md TASKS.md PHASE_REPORTS/phase-3.md TEST_RESULTS/phase-3.md
git commit -m "docs: record phase 3 validation"
```

Expected output contains `docs: record phase 3 validation`.

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
- agent dry run logs ETW counts and process analysis counts.

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

- `engine-process` crate: Task 1.
- Process state table: Task 1.
- Process start and exit lifecycle handling: Task 1.
- Suspicious parent-child signal: Task 2.
- PowerShell encoded command signal: Task 2.
- User-writable execution path signal: Task 2.
- Irrelevant telemetry ignored without panic: Tasks 1 and 2.
- Agent synthetic dry run: Task 3.
- Architecture validation and docs: Task 4.

Placeholder scan:

- The plan contains concrete file paths, commands, expected outputs, and code snippets for every behavior-changing step.

Type consistency:

- `ProcessStateTable::apply_event` returns `ProcessStateUpdate`.
- `ProcessAnalyzer::analyze` consumes `NormalizedTelemetryEvent` and returns `ProcessAnalysisReport`.
- `ProcessSignal` uses `SignalSeverity`, `ProcessSnapshot`, and `TelemetryEventId`.
- Agent dry run calls `run_synthetic_process_analysis_dry_run`.
