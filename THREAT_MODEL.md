# SentraEDR Threat Model

Date: 2026-06-27
Phase: 0

## Defender Goal

SentraEDR aims to detect and safely contain behavior associated with RATs, stealers, script-based execution, persistence, suspicious network activity, and later memory injection. The design maps behavior to MITRE ATT&CK techniques and validates coverage with controlled tests.

## Protected Assets

- Windows endpoint integrity.
- User files and credentials.
- Process execution history.
- Registry and persistence locations.
- Local telemetry database.
- Quarantine and rollback records.
- Detection rules and configuration.
- IPC command path between UI and agent.

## Adversary Behaviors In Scope

- Suspicious PowerShell execution.
- Script interpreter abuse.
- LOLBin execution chains.
- Persistence through Run keys, services, scheduled tasks, startup folders, and WMI.
- RAT-like outbound communication and beaconing.
- Data staging and credential theft indicators.
- DLL injection and suspicious memory permissions in later phases.
- Attempts to disable or overload telemetry.

## Adversary Behaviors Out Of Initial Scope

The following are explicitly deferred:

- kernel-mode rootkit detection;
- bootkit detection;
- cloud control plane monitoring;
- enterprise fleet management;
- ML anomaly modeling;
- automated destructive malware removal;
- live malware detonation.

These are deferred because Phase 0 and early implementation phases must first prove user-mode telemetry, detection correlation, and safety controls.

## MITRE ATT&CK Mapping

Initial technique families:

- Execution: PowerShell, command and scripting interpreters, signed binary proxy execution.
- Persistence: registry run keys, startup folder, scheduled task, service creation, WMI.
- Defense evasion: obfuscated command lines, suspicious parent process chains.
- Command and control: periodic outbound connections, unusual DNS, rare external endpoints.
- Credential access and collection: detected indirectly through process lineage and file/network behavior in early phases.

## Misuse Cases Against SentraEDR

Bypass telemetry:

- Attacker attempts to disable event providers or generate blind spots.
- Design response: provider health monitoring, dropped-event counters, uncertainty markers in findings.

Overload telemetry:

- Attacker floods low-value events to exhaust memory.
- Design response: bounded queues, priority drops, aggregation, pressure metrics.

Abuse remediation:

- Attacker tries to coerce the EDR into suspending safe processes.
- Design response: remediation requires policy checks, confidence, multi-signal evidence, and audit.

Abuse IPC:

- Attacker sends forged commands to the agent.
- Design response: pipe ACLs, schema validation, authorization, command audit.

Tamper with quarantine:

- Attacker attempts path traversal or restore abuse.
- Design response: canonical paths, restricted directory permissions, integrity metadata.

## Validation Strategy

Threat coverage is validated with:

- Atomic Red Team tests in isolated VMs;
- benign simulation tools created for the project;
- Windows Event Log and ETW replay where feasible;
- regression tests for scoring and false-positive suppression;
- manual review before enabling remediation.

## Phase 0 Status

This threat model defines expected adversary behaviors and safety constraints. Implementation begins in later phases.
