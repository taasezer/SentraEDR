# Risk Register

| ID | Description | Category | Mitigation Strategy |
|---|---|---|---|
| R-001 | ETW buffer exhaustion during telemetry spikes | Performance | Dedicate single OS thread strictly for ETW consumption. Use lock-free `tokio::sync::broadcast` to push backpressure outwards. |
| R-002 | Unbounded memory growth in Correlation State | Architecture | Hard cap of 50,000 events. Aggressive TTL purging mapped directly to active `Rule` max time windows. |
| R-003 | Remediation causing self-inflicted Denial of Service | Security/Platform | Default `Interactive` mode. Require explicit Human-in-the-Loop transition. Typestate enforcement. |
| R-004 | Async task starvation | Performance | Dedicated `Supervisor` layer wrapping tasks. Isolation of core platform threads from business logic threads. |
