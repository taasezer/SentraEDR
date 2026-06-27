# Detection Coverage Report

## Implemented Rule SDK Capabilities
The `core-rule-sdk` successfully enforces MITRE ATT&CK mapping via `RuleManifest`.

## Telemetry Requirements Map
- Process Telemetry: Supported natively. Rules mapped to Initial Access (T1078) and Execution (T1059) can now be enabled.
- Registry Telemetry: Supported natively. Rules mapped to Defense Evasion (T1112) can be enabled.
- Network Telemetry: Pending native ETW implementation. Rules mapped to Command and Control (T1071) will be rejected by the `RuleRegistry` capability validator until implemented.
