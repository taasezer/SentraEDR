# Phase 11 CI Quality Gates Design

Date: 2026-06-27
Status: Approved for implementation by project roadmap continuation

## Goal

Phase 11 adds repeatable CI quality gates for the Rust workspace. The same gate order can be inspected in code, run locally through PowerShell, and executed by GitHub Actions on the `Omer` branch and pull requests.

## Approach

The implementation has three layers:

- `testing-infra::quality_gate`: deterministic gate metadata for the required commands.
- `tools/run-quality-gates.ps1`: local runner for the same gate sequence.
- `.github/workflows/ci.yml`: GitHub Actions workflow using the same command order.

This phase does not add deployment, release publishing, signing, malware tests, VM orchestration, or destructive remediation tests.

## Required Gates

The default quality gate set runs:

1. `cargo fmt --all -- --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `powershell -ExecutionPolicy Bypass -File tools\validate-architecture.ps1`
5. `cargo run -p sentra-agent`

## Safety Rules

- CI gates must not execute malware or Atomic Red Team.
- CI gates must not mutate registry, firewall, quarantine, or host security state.
- CI gates must not run remediation executors.
- CI gates must not require secrets.
- CI gates must not push or publish artifacts.

## Testing

Tests cover:

- default gate order;
- all gates are non-destructive;
- required command strings are present;
- the gate set rejects destructive commands.

