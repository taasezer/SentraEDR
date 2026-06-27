# Phase 10 Testing Infrastructure Design

Date: 2026-06-27
Status: Approved for implementation by project roadmap continuation

## Goal

Phase 10 adds `testing-infra`, a safe test scenario catalog and phase coverage matrix. It documents which controlled scenarios validate each implemented engine without running malware, launching VM orchestration, executing Atomic Red Team, or touching host security controls.

## Approach

The crate is a deterministic test planning library. It represents safe synthetic scenarios, validates that unsafe scenarios are rejected, and produces a coverage summary for implemented phases.

This phase deliberately avoids:

- live malware execution;
- Atomic Red Team command execution;
- VM snapshot control;
- process suspension;
- registry writes;
- firewall changes;
- memory reads;
- IPC fuzzing against a live service.

## Components

`testing-infra::scenario`

- Owns `TestScenario`, `ScenarioKind`, and `SafetyLevel`.
- Defines safe scenario descriptors with phase coverage and MITRE technique tags.

`testing-infra::catalog`

- Owns `ScenarioCatalog`.
- Provides the default safe catalog for current phases.
- Rejects scenarios marked unsafe.

`testing-infra::matrix`

- Owns `CoverageMatrix` and `PhaseCoverage`.
- Counts scenarios per phase and identifies missing phase coverage.

## Initial Safe Scenarios

- Phase 2: synthetic process ETW lifecycle.
- Phase 3: PowerShell encoded command metadata.
- Phase 4: registry Run key persistence metadata.
- Phase 5: beacon-like network metadata.
- Phase 6: multi-signal detection correlation.
- Phase 7: approval-required remediation planning.
- Phase 8: remote thread memory metadata.
- Phase 9: dashboard alert summary projection.

## Safety Rules

- All default scenarios must be `SafetyLevel::Synthetic`.
- Unsafe scenarios are rejected before they enter a catalog.
- The crate must not import agent, UI, or engine crates.
- The crate must not run commands.
- The crate must not mutate host state.

## Testing

Tests cover:

- default catalog contains only safe synthetic scenarios;
- unsafe scenarios are rejected;
- implemented phases have coverage;
- missing phase coverage is reported;
- coverage report counts scenarios and MITRE tags deterministically.

