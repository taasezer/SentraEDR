# Phase 6 Heuristic Detection Design

Date: 2026-06-27
Status: Approved by continuous user instruction
Branch target: `Omer`

## Goal

Phase 6 adds `engine-detection`, an observe-only heuristic correlation engine that consumes normalized preliminary signals and emits scored findings plus observe-only alerts.

The engine depends on `shared-models` only. It must not import process, persistence, network, ETW, remediation, agent, or UI crates.

## Scope

Implemented:

- signal family modeling;
- severity hints;
- family diversity scoring;
- telemetry uncertainty marking;
- MITRE technique mapping for initial signal names;
- observe-only alert creation.

Out of scope:

- remediation eligibility;
- automatic quarantine or isolation;
- allowlist policy engine;
- ML anomaly scoring;
- production rule language;
- UI workflows.

## Scoring

Severity base:

- Low: 15
- Medium: 35
- High: 55

Correlation:

- add 15 points for each additional distinct signal family after the first;
- cap score at 100.

Risk levels:

- 0-19: Informational
- 20-39: Low
- 40-69: Medium
- 70-89: High
- 90-100: Critical

Findings are observe-only. Alerts created by Phase 6 must set `remediation_eligible` to `false`.
