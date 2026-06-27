# Phase 17 Report: Agent IPC Service Skeleton

## Summary
Phase 17 connects the previously completed in-memory IPC pipeline to `sentra-agent` through a small service lifecycle layer. The result is still transport-free and observe-only, but the agent can now accept IPC byte chunks in memory, dispatch decoded messages through bounded queues, and report IPC health counters during its synthetic startup dry run.

## Implemented Changes
- Added `IpcConfig` to `crates/sentra-agent/src/config.rs`.
- Added default IPC settings with `enabled = true` and `dispatcher_capacity = 256`.
- Added validation for zero IPC dispatcher capacity.
- Preserved TOML compatibility when the `[ipc]` section is omitted.
- Added `IpcService` in `crates/sentra-agent/src/ipc.rs`.
- Wrapped `shared_ipc::IpcPipeline` behind `process_raw_bytes`, `stats`, and dispatcher accessors.
- Added disabled-service behavior that ignores raw bytes without mutating IPC pipeline stats.
- Added synthetic IPC dry-run support in `crates/sentra-agent/src/ipc_dry_run.rs`.
- Updated agent startup logging to include IPC config and dry-run counters.
- Updated architecture, IPC design, performance, task, plan, report, and test-result documentation for Phase 17.

## Validation Results
- Config tests cover default IPC settings, TOML loading, omitted IPC defaults, and zero-capacity rejection.
- IPC service tests cover fragmented frame routing, disabled-service behavior, and invalid capacity rejection.
- IPC dry-run test covers encoded health message delivery through fragmented raw bytes.
- Full workspace tests, clippy, architecture validation, observe-only agent dry run, and the quality gate runner passed.

## Security Notes
- Phase 17 does not open named pipes, sockets, or any live transport.
- Phase 17 does not add UI command authorization, remediation execution, filesystem mutation, registry writes, firewall changes, malware execution, VM orchestration, deployment, or signing.
- Remediation-related messages remain data-only when routed through IPC primitives.

## Demo Impact
The agent now emits IPC counters in the same observe-only dry-run log as ETW, process, persistence, network, memory, detection, and remediation summaries. This gives the next phase a clean handoff point for live telemetry projection and UI-facing demo state without coupling UI code to engine internals.
