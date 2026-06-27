# Security Hardening Report

## Attack Surface Reduction
- **DLL Search Order:** Explicitly hardened using `SetDefaultDllDirectories` restricting to `LOAD_LIBRARY_SEARCH_SYSTEM32`.
- **Handle Inheritance:** Disabled for all file handlers to prevent leaking `TRACEHANDLE` or `EventBus` pipes to spawned child processes.
- **Temporary Files:** Enforced strict ACLs on `StorageProvider` ephemeral SQLite files.
