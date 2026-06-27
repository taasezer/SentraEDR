# Phase 3 Test Results

Date: 2026-06-27
Phase: Process monitoring engine

## Environment

Local validation used the Rust GNU Windows toolchain pinned by `rust-toolchain.toml`:

`stable-x86_64-pc-windows-gnu`

Known versions from earlier workspace validation:

- `rustc 1.96.0`
- `cargo 1.96.0`

## TDD Evidence

`cargo test -p engine-process --test state`

Initial result: Failed as expected before implementation because `ProcessLifecycleStatus`, `ProcessStateTable`, and `ProcessStateUpdate` were not exported. The first GREEN attempt also caught non-`Copy` `Timestamp` ownership errors, which were fixed with explicit clones.

Final result: Passed with 3 tests.

`cargo test -p engine-process --test signals`

Initial result: Failed as expected before implementation because `ProcessAnalyzer` and `SignalSeverity` were not exported.

Final result: Passed.

`cargo test -p sentra-agent --test process_dry_run`

Initial result: Failed as expected before implementation because `sentra_agent::process_dry_run` was missing.

Final result: Passed with 1 test.

## Final Commands

`cargo fmt --all -- --check`

Result: Passed after final formatting verification.

`cargo clippy --workspace --all-targets -- -D warnings`

Result: Passed.

`cargo test --workspace`

Result: Passed. Phase 3 coverage included:

- `process_start_inserts_running_snapshot`
- `process_exit_marks_existing_process_as_exited`
- `irrelevant_telemetry_is_ignored_without_state_change`
- `office_to_powershell_emits_suspicious_parent_child_signal`
- `powershell_encoded_command_emits_signal`
- `user_writable_execution_path_emits_signal`
- `non_process_event_is_counted_as_ignored`
- `synthetic_process_analysis_reports_signals`

`powershell -ExecutionPolicy Bypass -File tools/validate-architecture.ps1`

Result: Passed with `Architecture dependency validation passed.`

`cargo run -p sentra-agent`

Result: Passed. The binary logged observe-only ETW counts and process analysis counts including `process_observed=2`, `process_signals=2`, and `process_tracked=2`.

## Test Coverage

- `engine-process`: process lifecycle state, ignored telemetry, deterministic process behavior signals, and analysis reports.
- `sentra-agent`: synthetic process analysis dry-run integration.
- Architecture validation: dependency boundaries for `engine-process`.

## Validation Notes

- Phase 3 implements process state and preliminary signals only.
- Signals are not final findings, alerts, or remediation triggers.
- No real Windows process enumeration, signer check, persistent store, UI, detection scoring, or remediation path was implemented.
