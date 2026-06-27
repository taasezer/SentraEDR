# Phase 7 Remediation Engine Design

Date: 2026-06-27
Status: Approved for implementation by project roadmap continuation

## Goal

Phase 7 adds `engine-remediation`, a safe remediation planning engine. It evaluates alerts against explicit policy gates and emits auditable remediation decisions without performing destructive operating system actions.

## Approach

The engine is a policy and audit boundary, not an executor. It consumes shared `Alert` values and returns a `RemediationDecision` that records whether remediation is rejected, waiting for approval, or approved for a later executor phase.

Recommended default behavior remains observe-only:

- observe-only alerts are rejected by policy;
- alerts with telemetry uncertainty are rejected by policy;
- low and medium risk findings are rejected by policy;
- high and critical eligible alerts can produce approval-required plans;
- automatic completion, deletion, process suspension, firewall changes, registry writes, and quarantine moves are not implemented.

## Components

`engine-remediation::policy`

- Owns `RemediationPolicy`.
- Encodes mode, minimum risk level, manual approval requirement, and allowed remediation actions.

`engine-remediation::plan`

- Owns `RemediationPlan` and `PlannedRemediationStep`.
- Represents safe proposed steps such as suspend process, isolate network, quarantine file, and backup registry value.
- These are plans only. They do not call Windows APIs or modify the host.

`engine-remediation::audit`

- Owns `RemediationAuditRecord`.
- Records alert id, status, rationale, timestamp, mode, and planned step count.

`engine-remediation::engine`

- Owns `RemediationEngine`.
- Evaluates a single alert into a decision using policy gates and deterministic planning.

## Data Flow

```text
Alert
  -> RemediationEngine
  -> policy gates
  -> optional RemediationPlan
  -> RemediationAuditRecord
  -> RemediationDecision
```

## Safety Rules

- No direct dependency on other engine crates.
- No Windows API calls.
- No filesystem mutation.
- No registry mutation.
- No process suspension.
- No firewall modification.
- No deletion-first action.
- Every decision includes an audit record.

## Testing

Tests cover:

- observe-only alerts are rejected;
- telemetry uncertainty rejects remediation;
- high-risk eligible alerts create approval-required plans;
- disabled policy rejects all remediation;
- policy actions constrain generated plans.

