# SentraEDR Security Model

## Core Principles
1. **Quarantine-First Remediation:** The platform does not permanently delete files or kill essential system processes without a two-step validation. It isolates processes, suspends execution, and backs up registry entries.
2. **Detection vs. Remediation Separation:** The detection engine only generates verdicts. A separate module or human operator executes the remediation based on risk thresholds.
3. **Zero Trust IPC:** The `shared-ipc` layer treats all incoming messages as potentially malformed. Strict schema validation via `shared-models` is mandatory.
4. **Process Isolation:** The UI is completely isolated from the detection and telemetry engines to prevent web-based vulnerabilities (e.g., in Tauri) from compromising the core security functionality.

## Privilege Requirements
- The EDR engine runs as `SYSTEM` or `Administrator` to subscribe to critical ETW sessions and inspect memory.
- The UI dashboard can run with standard user privileges, communicating via secure IPC.

## Remediation Confidence
- **Score < 50:** Log only.
- **Score 50-84:** Alert generated, require human review.
- **Score >= 85:** Multi-signal agreement achieved (e.g., persistence + network beaconing), automatic process suspension and quarantine triggered.
