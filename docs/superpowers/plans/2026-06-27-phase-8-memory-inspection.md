# Phase 8 Memory Inspection Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add an observe-only memory telemetry analyzer that turns normalized memory metadata into preliminary injection and suspicious memory signals.

**Architecture:** `engine-memory` depends only on `shared-models`. It parses metadata from `TelemetryAction::MemoryEventObserved`, emits signals, and performs no OS memory inspection or remediation.

**Tech Stack:** Rust workspace crate, `shared-models`, deterministic unit tests, existing synthetic agent dry-run pattern.

---

### Task 1: Add Memory Analyzer Tests

**Files:**
- Create: `crates/engine-memory/tests/analyzer.rs`
- Modify later: `Cargo.toml`
- Create later: `crates/engine-memory/Cargo.toml`
- Create later: `crates/engine-memory/src/lib.rs`
- Create later: `crates/engine-memory/src/event.rs`
- Create later: `crates/engine-memory/src/signal.rs`
- Create later: `crates/engine-memory/src/analyzer.rs`

- [ ] **Step 1: Write failing tests**

```rust
use engine_memory::{MemoryAnalyzer, SignalSeverity};
use shared_models::{
    EventPriority, NormalizedTelemetryEvent, TelemetryAction, TelemetryMetadata, TelemetrySource,
    Timestamp,
};

#[test]
fn remote_thread_metadata_emits_high_signal() {
    let mut analyzer = MemoryAnalyzer::default();
    let report = analyzer.analyze(memory_event("remote_thread_created"));

    assert_eq!(report.stats.observed, 1);
    assert_eq!(report.stats.handled, 1);
    assert!(report.signals.iter().any(|s| s.name == "remote_thread_creation"));
    assert_eq!(report.signals[0].severity, SignalSeverity::High);
}

#[test]
fn executable_private_memory_emits_high_signal() {
    let mut analyzer = MemoryAnalyzer::default();
    let report = analyzer.analyze(
        memory_event("executable_private_memory")
            .with_confidence_hint(75),
    );

    assert!(report.signals.iter().any(|s| s.name == "executable_private_memory"));
}

#[test]
fn unsigned_module_metadata_emits_medium_signal() {
    let mut analyzer = MemoryAnalyzer::default();
    let report = analyzer.analyze(
        memory_event("unsigned_module_loaded")
            .with_confidence_hint(80),
    );

    let signal = report
        .signals
        .iter()
        .find(|s| s.name == "unsigned_module_loaded")
        .unwrap();
    assert_eq!(signal.severity, SignalSeverity::Medium);
}

#[test]
fn section_mapping_metadata_emits_signal() {
    let mut analyzer = MemoryAnalyzer::default();
    let report = analyzer.analyze(memory_event("section_mapping"));

    assert!(report.signals.iter().any(|s| s.name == "suspicious_section_mapping"));
}

#[test]
fn protection_change_to_execute_emits_signal() {
    let mut analyzer = MemoryAnalyzer::default();
    let report = analyzer.analyze(
        memory_event("protection_changed")
            .with_confidence_hint(90),
    );

    assert!(report.signals.iter().any(|s| s.name == "memory_protection_escalation"));
}

#[test]
fn non_memory_event_is_ignored() {
    let mut analyzer = MemoryAnalyzer::default();
    let event = NormalizedTelemetryEvent::new(
        TelemetrySource::WindowsEventLog,
        EventPriority::Low,
        TelemetryAction::ProcessStarted,
        ts(),
    );

    let report = analyzer.analyze(event);

    assert_eq!(report.stats.observed, 1);
    assert_eq!(report.stats.handled, 0);
    assert_eq!(report.stats.ignored, 1);
    assert!(report.signals.is_empty());
}

fn memory_event(kind: &str) -> NormalizedTelemetryEvent {
    let metadata = TelemetryMetadata::empty()
        .insert("memory.event_type", kind)
        .insert("memory.source_process_id", "4242")
        .insert("memory.target_process_id", "9001")
        .insert("memory.module_path", r"C:\Users\Public\stage.dll")
        .insert("memory.protection", "PAGE_EXECUTE_READWRITE")
        .insert("memory.region_kind", "private")
        .insert("memory.allocation_size", "4096")
        .insert("memory.thread_start_address", "0x1000");

    let mut event = NormalizedTelemetryEvent::new(
        TelemetrySource::Etw,
        EventPriority::High,
        TelemetryAction::MemoryEventObserved,
        ts(),
    );
    event.metadata = metadata;
    event
}

fn ts() -> Timestamp {
    Timestamp::parse_rfc3339("2026-06-27T09:09:00Z").unwrap()
}
```

- [ ] **Step 2: Run RED**

Run: `cargo test -p engine-memory --test analyzer`

Expected: FAIL because `engine-memory` is not a workspace package.

### Task 2: Implement Memory Engine

**Files:**
- Modify: `Cargo.toml`
- Modify: `crates/shared-models/src/telemetry.rs`
- Create: `crates/engine-memory/Cargo.toml`
- Create: `crates/engine-memory/src/lib.rs`
- Create: `crates/engine-memory/src/event.rs`
- Create: `crates/engine-memory/src/signal.rs`
- Create: `crates/engine-memory/src/analyzer.rs`

- [ ] **Step 1: Add `TelemetryAction::MemoryEventObserved`**

Add a new shared telemetry action for normalized memory metadata events.

- [ ] **Step 2: Add workspace crate**

Add `"crates/engine-memory"` to the workspace members.

- [ ] **Step 3: Implement event parsing and signal mapping**

Implement `MemoryEvent::from_telemetry`, `MemoryEventKind`, `MemoryAnalyzer`, `MemoryAnalysisReport`, `MemoryAnalysisStats`, `MemorySignal`, and `SignalSeverity`.

- [ ] **Step 4: Run GREEN**

Run: `cargo test -p engine-memory --test analyzer`

Expected: PASS with 6 tests.

### Task 3: Add Agent Dry Run

**Files:**
- Modify: `crates/sentra-agent/Cargo.toml`
- Create: `crates/sentra-agent/src/memory_dry_run.rs`
- Modify: `crates/sentra-agent/src/lib.rs`
- Modify: `crates/sentra-agent/src/main.rs`
- Create: `crates/sentra-agent/tests/memory_dry_run.rs`

- [ ] **Step 1: Write failing test**

```rust
#[test]
fn synthetic_memory_analysis_reports_signals() {
    let report = sentra_agent::memory_dry_run::run_synthetic_memory_analysis_dry_run();

    assert_eq!(report.stats.observed, 3);
    assert_eq!(report.stats.handled, 3);
    assert!(report.signals.iter().any(|s| s.name == "remote_thread_creation"));
    assert!(report.signals.iter().any(|s| s.name == "executable_private_memory"));
    assert!(report.signals.iter().any(|s| s.name == "memory_protection_escalation"));
}
```

- [ ] **Step 2: Run RED**

Run: `cargo test -p sentra-agent --test memory_dry_run`

Expected: FAIL because `sentra_agent::memory_dry_run` is missing.

- [ ] **Step 3: Implement dry run and logging**

Create three synthetic memory metadata events and log observed, handled, and signal counts in `sentra-agent`.

- [ ] **Step 4: Run GREEN**

Run: `cargo test -p sentra-agent --test memory_dry_run`

Expected: PASS with 1 test.

### Task 4: Documentation And Final Verification

**Files:**
- Modify: `tools/validate-architecture.ps1`
- Modify: `ARCHITECTURE.md`
- Modify: `PERFORMANCE_NOTES.md`
- Modify: `TASKS.md`
- Create: `PHASE_REPORTS/phase-8.md`
- Create: `TEST_RESULTS/phase-8.md`

- [ ] **Step 1: Add architecture validation rules**

Add rules that prevent `engine-memory` from depending on agent, UI, or peer engine crates.

- [ ] **Step 2: Update phase documentation**

Record Phase 8 as safe metadata-only memory telemetry analysis.

- [ ] **Step 3: Run final verification**

Run:

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
powershell -ExecutionPolicy Bypass -File tools\validate-architecture.ps1
cargo run -p sentra-agent
```

Expected: all commands exit 0.

