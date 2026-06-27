# ADR 0006: Remediation State Machine & Safety

## Status
Accepted

## Context
Remediation actions manipulate host state (e.g., terminating processes, deleting files). If these actions are executed implicitly or skip safety checks, the EDR risks becoming a tool for self-inflicted denial of service. The logic must be strictly ordered, idempotent, and reversible.

## Decision
The Remediation Engine will implement an explicit State Machine modeled as a Rust `enum` that consumes its previous state. The transitions are mathematically constrained:
- `AlertReceived` -> `ActionPlanned`
- `ActionPlanned` -> `ActionPlanPendingApproval` (if Interactive) OR `SafetyValidated` (if Automatic)
- `SafetyValidated` -> `Executing`
- `Executing` -> `Verification`
- `Verification` -> `Completed` | `Failed` | `RolledBack`

Illegal transitions (e.g., executing without validation) are impossible because the functions accepting these states require the specific previous `Enum` variant as input.

## Alternatives Considered
- **Implicit Workflow:** A single `process_alert` function with internal `if/else` checks. Rejected because adding new rules or providers inevitably introduces edge cases that skip checks.
- **Direct Execution:** Detection engine calls `KillProcess()`. Rejected because it merges detection and response, violating the core architecture.

## Trade-offs
- *Pros:* Complete type-safety for remediation. 100% guarantee that Rollback data is generated before execution.
- *Cons:* Slightly higher boilerplate in `pipeline.rs`.

## Consequences
Every remediation action must pass through the `ActionRegistry` and fulfill the `ActionProvider` trait, providing explicit `verify()` and `generate_rollback()` logic before it can be added to the state machine.
