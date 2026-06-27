# Phase 10 Testing Infrastructure Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a safe test scenario catalog and phase coverage matrix for implemented SentraEDR phases.

**Architecture:** `testing-infra` is a planning crate with no dependency on agent, UI, or engine crates. It models safe synthetic scenarios and coverage reports without executing host actions.

**Tech Stack:** Rust workspace crate, deterministic unit tests, existing architecture validation script.

---

### Task 1: Add Scenario Catalog Tests

**Files:**
- Create: `crates/testing-infra/tests/catalog.rs`
- Modify later: `Cargo.toml`
- Create later: `crates/testing-infra/Cargo.toml`
- Create later: `crates/testing-infra/src/lib.rs`
- Create later: `crates/testing-infra/src/scenario.rs`
- Create later: `crates/testing-infra/src/catalog.rs`
- Create later: `crates/testing-infra/src/matrix.rs`

- [ ] **Step 1: Write failing tests**

```rust
use testing_infra::{
    CoverageMatrix, SafetyLevel, ScenarioCatalog, ScenarioKind, TestScenario,
};

#[test]
fn default_catalog_contains_only_synthetic_safe_scenarios() {
    let catalog = ScenarioCatalog::default_safe();

    assert!(catalog.scenarios.len() >= 8);
    assert!(
        catalog
            .scenarios
            .iter()
            .all(|scenario| scenario.safety == SafetyLevel::Synthetic)
    );
}

#[test]
fn unsafe_scenarios_are_rejected() {
    let result = ScenarioCatalog::try_new(vec![TestScenario::new(
        "live malware sample",
        ScenarioKind::MalwareExecution,
        SafetyLevel::Unsafe,
        vec![10],
        vec!["T0000"],
    )]);

    assert!(result.is_err());
}

#[test]
fn coverage_matrix_reports_implemented_phase_coverage() {
    let catalog = ScenarioCatalog::default_safe();
    let matrix = CoverageMatrix::from_catalog(&catalog, 2..=9);

    assert!(matrix.missing_phases.is_empty());
    assert_eq!(matrix.coverage_for_phase(2).unwrap().scenario_count, 1);
    assert_eq!(matrix.coverage_for_phase(9).unwrap().scenario_count, 1);
}

#[test]
fn coverage_matrix_reports_missing_phase() {
    let catalog = ScenarioCatalog::try_new(vec![TestScenario::new(
        "synthetic process ETW lifecycle",
        ScenarioKind::TelemetryReplay,
        SafetyLevel::Synthetic,
        vec![2],
        vec!["T1059"],
    )])
    .unwrap();

    let matrix = CoverageMatrix::from_catalog(&catalog, 2..=3);

    assert_eq!(matrix.missing_phases, vec![3]);
}

#[test]
fn coverage_report_counts_scenarios_and_mitre_tags() {
    let catalog = ScenarioCatalog::default_safe();
    let matrix = CoverageMatrix::from_catalog(&catalog, 2..=9);
    let report = matrix.report();

    assert_eq!(report.total_phases, 8);
    assert_eq!(report.covered_phases, 8);
    assert!(report.total_scenarios >= 8);
    assert!(report.unique_mitre_tags >= 4);
}
```

- [ ] **Step 2: Run RED**

Run: `cargo test -p testing-infra --test catalog`

Expected: FAIL because `testing-infra` is not a workspace package.

### Task 2: Implement Testing Infrastructure Crate

**Files:**
- Modify: `Cargo.toml`
- Create: `crates/testing-infra/Cargo.toml`
- Create: `crates/testing-infra/src/lib.rs`
- Create: `crates/testing-infra/src/scenario.rs`
- Create: `crates/testing-infra/src/catalog.rs`
- Create: `crates/testing-infra/src/matrix.rs`

- [ ] **Step 1: Add workspace crate**

Add `"crates/testing-infra"` to workspace members.

- [ ] **Step 2: Implement scenario models**

Implement `TestScenario`, `ScenarioKind`, and `SafetyLevel`.

- [ ] **Step 3: Implement safe catalog**

Implement `ScenarioCatalog::try_new` and `ScenarioCatalog::default_safe`.

- [ ] **Step 4: Implement coverage matrix**

Implement `CoverageMatrix`, `PhaseCoverage`, and `CoverageReport`.

- [ ] **Step 5: Run GREEN**

Run: `cargo test -p testing-infra --test catalog`

Expected: PASS with 5 tests.

### Task 3: Architecture And Documentation

**Files:**
- Modify: `tools/validate-architecture.ps1`
- Modify: `ARCHITECTURE.md`
- Modify: `PERFORMANCE_NOTES.md`
- Modify: `TASKS.md`
- Create: `PHASE_REPORTS/phase-10.md`
- Create: `TEST_RESULTS/phase-10.md`

- [ ] **Step 1: Add architecture validation rules**

Add rules that prevent `testing-infra` from depending on agent, UI, IPC, or engine crates.

- [ ] **Step 2: Update phase documentation**

Record Phase 10 as safe testing infrastructure only. Document that no live malware, Atomic Red Team execution, VM orchestration, remediation execution, or host mutation is implemented.

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

