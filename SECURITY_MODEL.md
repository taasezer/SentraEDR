# SentraEDR Security Model

Date: 2026-06-27
Phase: 0

## Security Objective

SentraEDR must detect suspicious behavior without becoming a source of operating-system instability or a local privilege escalation path. The default development posture is observe-only until the remediation pipeline has confidence gates, rollback, and audit controls.

## Trust Boundaries

Agent process:

- Trusted to collect telemetry and enforce local policy.
- Must validate all external input.

UI process:

- Trusted for presentation and user intent only.
- Not trusted to bypass agent policy.

Named pipe IPC:

- Local boundary between UI and agent.
- Requires restrictive ACLs, schema validation, command authorization, and audit logging.

Telemetry parsers:

- Treat all OS event payloads as untrusted data.
- Must defend against malformed event payloads, unexpected encodings, and missing fields.

Quarantine and rollback storage:

- Security-sensitive local storage.
- Must prevent arbitrary overwrite, path traversal, and unauthorized restore.

Rule/config loading:

- Security-sensitive input.
- Must be signed or integrity-checked before production use.

## Critical Failure Modes

Telemetry overload:

- Risk: memory growth, delayed detection, dropped critical events.
- Control: bounded queues, priority routing, low-priority shedding, queue health metrics.

Wrong remediation:

- Risk: legitimate process suspension, registry corruption, file loss, OS instability.
- Control: observe-only default, multi-signal confidence, quarantine-first actions, rollback snapshots, user approval.

Crate coupling breakdown:

- Risk: hidden side effects, circular dependencies, untestable security logic.
- Control: strict dependency rules, shared schema layer, IPC-mediated communication.

## Remediation Safety

The first remediation-capable implementation must follow this order:

1. Produce detection verdict.
2. Verify confidence threshold and signal diversity.
3. Record audit decision.
4. Create rollback material when registry or file state may change.
5. Suspend or isolate before quarantine where policy allows.
6. Quarantine before deletion.
7. Require explicit user approval for critical actions in real deployment mode.

Immediate deletion is outside the default policy.

## Attack Surface

Primary attack surfaces:

- named pipe server;
- UI command input;
- telemetry event parsers;
- configuration and rule files;
- quarantine restore path;
- SQLite database;
- Windows service control and installation path;
- logging and crash-report files.

## Secure Defaults

- Observe-only mode is default.
- Remediation is disabled until Phase 7.
- IPC commands require schema version checks.
- Unknown schema versions are rejected.
- Logs must avoid secrets, tokens, full credential material, and excessive command-line leakage.
- Test payloads are limited to safe simulators, EICAR where appropriate, and Atomic Red Team in isolated VMs.

## Phase 0 Status

The security model is a design contract. No runtime controls are implemented in Phase 0.
