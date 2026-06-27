# Installation Operations Guide

## Lifecycle Flows
1. **Install Flow:** MSI executes. Checks compatibility. Elevates. Copies binaries to `%PROGRAMFILES%\SentraEDR`. Registers `service-host` via `sc.exe create`.
2. **Upgrade Flow:** SCM issues `SERVICE_CONTROL_STOP`. Existing process drains queues and exits gracefully. MSI copies new binaries. `service-host` starts. `core-runtime` executes `.sql` and `.json` schema migration hooks before declaring `Ready`.
3. **Uninstall Flow:** Agent drops ETW subscriptions. SCM stops service. Binaries removed. Logs and quarantine storage remain unless explicit `--purge` flag is provided.

## Version Compatibility
The `RuntimeBuilder` validates Rule Pack SDK bounds against the Agent version on startup to prevent executing incompatible detection rules.
