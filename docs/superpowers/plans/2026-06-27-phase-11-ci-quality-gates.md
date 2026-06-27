# Phase 11 CI Quality Gates Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add local and GitHub CI quality gates matching the verified workspace commands.

**Architecture:** Quality gate metadata lives in `testing-infra`; the PowerShell runner and GitHub Actions workflow execute the same command order. No gate executes malware, remediation, deployment, signing, or host mutation.

**Tech Stack:** Rust `testing-infra`, PowerShell, GitHub Actions, existing Rust workspace commands.

---

### Task 1: Add Quality Gate Metadata Tests

**Files:**
- Create: `crates/testing-infra/tests/quality_gate.rs`
- Modify later: `crates/testing-infra/src/lib.rs`
- Create later: `crates/testing-infra/src/quality_gate.rs`

- [x] **Step 1: Write failing tests**

```rust
use testing_infra::{QualityGateCommand, QualityGateSet};

#[test]
fn default_quality_gates_are_in_required_order() {
    let gates = QualityGateSet::default_workspace();
    let names: Vec<&str> = gates.commands.iter().map(|gate| gate.name.as_str()).collect();

    assert_eq!(
        names,
        vec![
            "format",
            "clippy",
            "workspace-tests",
            "architecture-validation",
            "agent-dry-run"
        ]
    );
}

#[test]
fn default_quality_gates_include_required_commands() {
    let gates = QualityGateSet::default_workspace();
    let commands: Vec<String> = gates.commands.iter().map(|gate| gate.command_line()).collect();

    assert!(commands.iter().any(|cmd| cmd == "cargo fmt --all -- --check"));
    assert!(commands.iter().any(|cmd| cmd == "cargo clippy --workspace --all-targets -- -D warnings"));
    assert!(commands.iter().any(|cmd| cmd == "cargo test --workspace"));
    assert!(commands.iter().any(|cmd| cmd.contains("tools\\validate-architecture.ps1")));
    assert!(commands.iter().any(|cmd| cmd == "cargo run -p sentra-agent"));
}

#[test]
fn default_quality_gates_are_non_destructive() {
    let gates = QualityGateSet::default_workspace();

    assert!(gates.commands.iter().all(|gate| gate.destructive == false));
    assert!(gates.validate_safe().is_ok());
}

#[test]
fn destructive_quality_gate_is_rejected() {
    let gates = QualityGateSet::try_new(vec![QualityGateCommand::new(
        "delete",
        "Remove-Item",
        vec!["-Recurse", "C:\\"],
        true,
    )]);

    assert!(gates.is_err());
}
```

- [x] **Step 2: Run RED**

Run: `cargo test -p testing-infra --test quality_gate`

Expected: FAIL because `QualityGateSet` and `QualityGateCommand` are missing.

### Task 2: Implement Quality Gate Metadata

**Files:**
- Modify: `crates/testing-infra/src/lib.rs`
- Create: `crates/testing-infra/src/quality_gate.rs`

- [x] **Step 1: Implement `QualityGateCommand`**

Fields: name, program, args, destructive.

- [x] **Step 2: Implement `QualityGateSet`**

Methods: `try_new`, `default_workspace`, `validate_safe`.

- [x] **Step 3: Run GREEN**

Run: `cargo test -p testing-infra --test quality_gate`

Expected: PASS with 4 tests.

### Task 3: Add Local Runner And GitHub Workflow

**Files:**
- Create: `tools/run-quality-gates.ps1`
- Create: `.github/workflows/ci.yml`

- [x] **Step 1: Add PowerShell runner**

The script sets `$ErrorActionPreference = "Stop"` and runs the required commands in order.

- [x] **Step 2: Add GitHub Actions workflow**

Workflow runs on `push` to `Omer` and `pull_request`, uses Windows latest, installs stable Rust, then runs the same command sequence.

- [x] **Step 3: Smoke-test local runner**

Run: `powershell -ExecutionPolicy Bypass -File tools\run-quality-gates.ps1`

Expected: all gates exit 0.

### Task 4: Documentation And Final Verification

**Files:**
- Modify: `ARCHITECTURE.md`
- Modify: `PERFORMANCE_NOTES.md`
- Modify: `TASKS.md`
- Create: `PHASE_REPORTS/phase-11.md`
- Create: `TEST_RESULTS/phase-11.md`

- [x] **Step 1: Update phase documentation**

Record Phase 11 as local/CI quality gates only.

- [x] **Step 2: Run final verification**

Run:

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
powershell -ExecutionPolicy Bypass -File tools\validate-architecture.ps1
cargo run -p sentra-agent
powershell -ExecutionPolicy Bypass -File tools\run-quality-gates.ps1
```

Expected: all commands exit 0.
