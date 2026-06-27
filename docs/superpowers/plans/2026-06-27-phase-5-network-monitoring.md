# Phase 5 Network Monitoring Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add `engine-network`, a metadata-only observe-only network analyzer with preliminary network behavior signals.

**Architecture:** `engine-network` consumes `NormalizedTelemetryEvent` and `TelemetryMetadata` only. It owns network metadata parsing, small in-memory destination history, signal matching, and analysis reports; `sentra-agent` only runs a synthetic dry run and logs counts.

**Tech Stack:** Rust 1.85+ edition 2024, existing workspace crates, `shared-models`, local GNU Windows toolchain.

---

## Tasks

- [ ] Add `engine-network` workspace crate with analyzer tests first.
- [ ] Implement network event parsing, destination history, and five signals.
- [ ] Add agent synthetic network dry run and test.
- [ ] Add architecture validation, Phase 5 docs, final verification, and push to `Omer`.

## Verification

Run:

```powershell
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
powershell -ExecutionPolicy Bypass -File tools\validate-architecture.ps1
cargo run -p sentra-agent
```

Expected: all commands pass, and agent logs `network_observed`, `network_handled`, and `network_signals`.
