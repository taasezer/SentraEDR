# ADR 0031: Rule Lifecycle and Budgets

## Status
Accepted

## Decision
Because Rust does not support preemptive thread termination for out-of-budget allocations without custom allocators, we enforce the `PerformanceBudget` through **Hybrid Verification**. Hard enforcement happens during CI via the `ScenarioRunner`. In Production, the `RuleProfiler` tracks execution times and emits Diagnostic Reports if a rule exceeds its budget, favoring observability over abrupt detection termination.
