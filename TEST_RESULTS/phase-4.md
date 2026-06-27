# Phase 4 Test Results

Date: 2026-06-27
Phase: Persistence monitoring engine

## Environment

Local validation used the Rust GNU Windows toolchain pinned by `rust-toolchain.toml`:

`stable-x86_64-pc-windows-gnu`

Known versions from earlier workspace validation:

- `rustc 1.96.0`
- `cargo 1.96.0`

## TDD Evidence

`cargo test -p engine-persistence --test analyzer`

Initial result: Failed as expected before implementation because `PersistenceAnalyzer`, `PersistenceKind`, and `SignalSeverity` were not exported.

Final result: Passed with 6 tests.

`cargo test -p sentra-agent --test persistence_dry_run`

Initial result: Failed as expected before implementation because `sentra_agent::persistence_dry_run` was missing. The first GREEN attempt exposed that analyzer reports per-event signals while the dry run expected aggregate signals, so aggregation was fixed in the dry-run boundary.

Final result: Passed with 1 test.

## Final Commands

`cargo fmt --all -- --check`

Result: Passed after final formatting verification.

`cargo clippy --workspace --all-targets -- -D warnings`

Result: Passed.

`cargo test --workspace`

Result: Passed. Phase 4 coverage included:

- `run_key_metadata_emits_registry_run_key_signal`
- `startup_folder_metadata_emits_startup_folder_signal`
- `scheduled_task_metadata_emits_scheduled_task_signal`
- `service_metadata_emits_service_signal`
- `wmi_metadata_emits_wmi_signal`
- `irrelevant_telemetry_is_counted_as_ignored`
- `synthetic_persistence_analysis_reports_signals`

`powershell -ExecutionPolicy Bypass -File tools/validate-architecture.ps1`

Result: Passed with `Architecture dependency validation passed.`

`cargo run -p sentra-agent`

Result: Passed. The binary logged observe-only ETW, process, and persistence counts including `persistence_observed=2`, `persistence_handled=2`, and `persistence_signals=2`.

## Test Coverage

- `engine-persistence`: metadata parsing, persistence kind classification, deterministic persistence behavior signals, ignored telemetry, and analysis reports.
- `sentra-agent`: synthetic persistence analysis dry-run integration.
- Architecture validation: dependency boundaries for `engine-persistence`.

## Validation Notes

- Phase 4 implements metadata-only persistence analysis.
- Signals are not final findings, alerts, rollback requests, or remediation triggers.
- No Windows Registry API access, scheduled task API access, service control API access, WMI querying, filesystem scanning, UI, detection scoring, or remediation path was implemented.
