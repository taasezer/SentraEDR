# Remediation Engine Architecture

The Remediation Engine is the isolated component responsible for altering OS state in response to immutable `Alert`s.

## Strict State Machine
The remediation lifecycle is implemented via the typestate pattern. Transitions are explicit:
1. `AlertReceived`
2. `ActionPlanned` (ActionId generated, decoupled from AlertId)
3. `ActionPlanPendingApproval` (Human-in-the-loop enforcement)
4. `Approved` / `Rejected`
5. `SafetyValidated` (Policy evaluation)
6. `Executing` (Delegation to ActionProvider)
7. `Verification` (Mandatory confirmation that the OS state matches the expected result)
8. `Completed` / `Failed` / `RolledBack`

## ActionRegistry & Providers
Remediation logic is NEVER hardcoded in the pipeline. Instead, the engine relies on an `ActionRegistry`.
- An `ActionProvider` abstracts the OS mechanics (e.g., `ProcessActionProvider`).
- Providers must implement `execute`, `verify`, `generate_rollback`, `dry_run`, and `is_idempotent`.
- The engine can operate fully in "Dry Run" mode to test rules without touching the host.

## Rollback & Idempotency
- **Rollback Independence:** The `RollbackData` model is entirely self-contained. It contains the literal state needed to reverse an action without querying the provider again.
- **Idempotency:** Providers declare idempotency. If an action fails verification and is retried, the system guarantees safety.

## Immutable Audit Trail
Every final state transition (`Completed`, `Failed`, `RolledBack`) emits an immutable `AuditRecord` domain event.
This record includes cryptographic integrity placeholders (e.g. SHA-256 hashes of the payload) ensuring the audit trail cannot be silently altered before being ingested by the future Storage layer.
