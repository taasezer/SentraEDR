# Technical Debt Register

| ID | Description | Impact | Affected Modules | Risk Level | Proposed Solution | Target Phase |
|---|---|---|---|---|---|---|
| TD-001 | Mock Action Providers used for Remediation tests | Cannot execute real OS actions yet | `engine-remediation` | Low | Implement `Win32ProcessProvider` | Post-Phase 10 |
| TD-002 | SQLite bindings missing in Storage | Cannot persist data across reboots | `infrastructure-storage` | Medium | Implement `rusqlite` `StorageProvider` | Post-Phase 10 |
