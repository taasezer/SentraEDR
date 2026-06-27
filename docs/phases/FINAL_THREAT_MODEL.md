# Final Threat Model

## Production Scenarios
- **Privilege Escalation:** SentraEDR drops execution privileges for child components where possible. Minimum privilege principles are mapped by the `PrivilegeValidator`.
- **DLL Hijacking:** Mitigated by `LOAD_LIBRARY_SEARCH_SYSTEM32` bounds in `service-host`.
- **IPC Abuse:** Both Local IPC and Tauri IPC explicitly validate all payloads and drop malformed data.
- **Malicious Rules:** `RuleSDK` enforces strict performance and allocation budgets. A rule cannot arbitrarily modify the host machine.
- **Compromised Plugins:** Future plugins must conform to signature checks and run inside strict typestate sandboxes.
