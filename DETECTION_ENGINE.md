# SentraEDR Detection Engine

Date: 2026-06-27
Phase: 0

## Detection Philosophy

SentraEDR detects behavior chains rather than relying only on signatures. A single suspicious event should usually produce context, not immediate remediation. High-impact alerts require multiple supporting signals.

## Initial Signal Families

Process signals:

- unusual parent-child process chains;
- suspicious executable paths;
- script interpreters launched by Office, browsers, archive tools, or unknown parents;
- hidden windows or suspicious command-line flags;
- unsigned or rare executables once metadata collection exists.

PowerShell signals:

- encoded command usage;
- execution policy bypass;
- remote download and execution patterns;
- suspicious child process creation;
- high-risk flags combined with unusual parent processes.

Persistence signals:

- Run and RunOnce key changes;
- startup folder file creation;
- scheduled task creation or modification;
- service creation or path change;
- WMI permanent event subscription indicators.

Network signals:

- new or rare outbound destinations;
- repeated interval-like connection cadence;
- suspicious DNS patterns;
- network activity shortly after risky process creation.

Memory signals:

- suspicious thread starts;
- RWX memory regions;
- injected module indicators.

Memory signals are deferred until the ETW, process, persistence, and detection pipelines are stable.

## Scoring Model

The detection engine should score findings using:

- signal severity;
- signal confidence;
- process reputation context;
- signer and path trust;
- parent-child lineage;
- timing correlation;
- user or system context;
- telemetry uncertainty.

Risk levels:

- Informational: visible in telemetry, no alert.
- Low: weak signal or known benign context.
- Medium: meaningful suspicious behavior without enough correlation for action.
- High: multiple suspicious signals with good confidence.
- Critical: high-confidence multi-signal behavior eligible for gated remediation.

## False-Positive Controls

Controls:

- require signal correlation for high severity;
- suppress known safe Windows paths and signed system binaries where context is normal;
- include telemetry uncertainty in confidence;
- allow policy-based allowlists with audit history;
- prefer observe-only alerts before remediation;
- record why a score was assigned.

## Alert Contract

An alert should contain:

- schema version;
- alert ID;
- timestamp;
- risk level and score;
- affected process and lineage;
- ATT&CK mapping where applicable;
- supporting signals;
- telemetry uncertainty;
- recommended action;
- remediation eligibility.

## Remediation Interface

The detection engine never directly kills, deletes, quarantines, or edits registry state. It emits verdicts. The remediation layer decides whether an action is allowed by policy and user approval.

## Phase 0 Status

Detection logic is specified but not implemented. Phase 6 is the first full scoring and correlation phase. Earlier phases may emit preliminary findings in observe-only mode.
