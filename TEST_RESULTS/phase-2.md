# Phase 2 Test Results

Date: 2026-06-27
Phase: ETW process telemetry ingestion

## Environment

Local validation used the Rust GNU Windows toolchain pinned by `rust-toolchain.toml`:

`stable-x86_64-pc-windows-gnu`

Known versions from earlier workspace validation:

- `rustc 1.96.0`
- `cargo 1.96.0`

## TDD Evidence

`cargo test -p engine-etw --test normalizer`

Initial result: Failed as expected before implementation because `engine-etw` did not yet expose `EtwProcessEventKind`, `EtwProcessRecord`, or `normalize_process_record`.

Final result: Passed with 2 tests.

`cargo test -p engine-etw --test ingestion`

Initial result: Failed as expected before implementation because `EtwIngestor`, `SyntheticEtwSource`, and `BoundedReceiver::try_recv` were missing.

Final result: Passed.

`cargo test -p sentra-agent --test dry_run`

Initial result: Failed as expected before implementation because `sentra_agent::dry_run` was missing.

Final result: Passed with 1 test.

## Final Commands

`cargo fmt --all -- --check`

Result: Initially reported two formatting diffs. `cargo fmt --all` was applied, then the check passed.

`cargo clippy --workspace --all-targets -- -D warnings`

Result: Passed.

`cargo test --workspace`

Result: Passed. Phase 2 coverage included:

- `synthetic_source_drains_into_bounded_queue`
- `queue_pressure_degrades_component_health`
- `process_start_record_normalizes_to_telemetry_event`
- `process_exit_record_normalizes_to_low_priority_exit_event`
- `synthetic_etw_dry_run_reports_two_normalized_events`

`powershell -ExecutionPolicy Bypass -File tools/validate-architecture.ps1`

Result: Passed with `Architecture dependency validation passed.`

`cargo run -p sentra-agent`

Result: Passed. The binary logged observe-only mode with `etw_received=2`, `etw_normalized=2`, and `etw_dropped=0`.

## Test Coverage

- `engine-etw`: process lifecycle normalization, synthetic source draining, bounded queue delivery, dropped-event accounting, and health degradation.
- `shared-ipc`: deterministic non-blocking receive through `try_recv` and bounded queue depth/drop behavior.
- `sentra-agent`: synthetic ETW dry-run integration and observe-only logging path.

## Validation Notes

- Phase 2 implements a portable synthetic ingestion path only.
- No real Windows ETW session, callback loop, provider registration, Windows service, detection engine, remediation executor, or UI streaming was implemented.
- Queue pressure is intentionally non-fatal and observable through metrics.
