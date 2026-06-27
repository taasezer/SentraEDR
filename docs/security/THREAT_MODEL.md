# SentraEDR Threat Model

## Trust Boundaries
- **Kernel/OS Layer:** Fully trusted. We rely on ETW and the NT Kernel for accurate telemetry.
- **Agent Engines:** Highly trusted. Must be protected against memory corruption (using Rust) and logic bugs.
- **UI Dashboard:** Least trusted. Must not be able to compromise the detection engine or execute arbitrary remediation.

## Assumed Attack Vectors Against SentraEDR
1. **Telemetry Flooding:** Attackers intentionally spam ETW (e.g., rapid process creation or registry edits) to cause resource exhaustion or drop critical events.
2. **Process Injection/Tampering:** Attempting to inject code into the SentraEDR agent or terminate the service.
3. **ETW Tampering:** Attackers using rootkits or advanced driver manipulation to disable or filter ETW providers.
4. **IPC Hijacking:** Malicious processes attempting to connect to the SentraEDR named pipe to spoof telemetry or alerts.

## Mitigation Strategies
- **Bounded Queues & Backpressure:** Mitigates telemetry flooding by dropping low-priority events before high-priority ones, ensuring stability.
- **Rust Memory Safety:** Prevents traditional buffer overflows and use-after-free vulnerabilities within the agent.
- **Strict IPC Validation:** Named pipes will require strict access controls (ACLs) and message schema validation to prevent unauthorized access.
- **Self-Protection:** Future kernel integrations or service configuration hardening to prevent unauthorized termination.
