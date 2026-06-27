# Engineering Guidelines

SentraEDR is now in Maintenance Mode.

## When is a new crate allowed?
Only for new `infrastructure-providers` (e.g., `infrastructure-network-ebpf`), new `engine-telemetry` sources, or decoupled `ui` plugins. No new `core-*` crates may be created without a formal Architecture Review.

## ADR Requirements
Every structural change, new provider, or IPC boundary modification REQUIRES an Architecture Decision Record (ADR).

## Benchmarking & Testing
Any new Rule added to `core-rule-sdk` MUST provide its Performance Budget bounds and execute against the `core-testkit` Replay Harness in CI. Code coverage must remain above 95% for detection logic.
