# ADR 0030: Rule SDK Architecture

## Status
Accepted

## Decision
Rules are decoupled from the Detection Engine via `core-rule-sdk`. Each rule exposes a `RuleManifest` declaring MITRE ATT&CK coverage, required capabilities, and its `PerformanceBudget`. The `RuleRegistry` enables rules to be hot-swapped or packaged independently from the core platform compilation.
