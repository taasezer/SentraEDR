# Persistence Monitoring Architecture

The Persistence Monitoring Engine (`engine-persistence`) evaluates Auto-Start Extensibility Points (ASEPs) purely for observation. It explicitly restricts its privileges to read-only actions and delegates all threat classification to the downstream Detection Engine.

## Polling vs Event-Driven Sources
Persistence mechanisms operate under two different telemetry paradigms:

1. **Event-Driven (Real-time):**
   - *Registry (Run/RunOnce, IFEO):* Monitored dynamically via ETW `RegistryActivity` events. The engine reacts immediately.
   - *Services:* Monitored dynamically via ETW `ProcessCreate` (sc.exe) or Event Log subscriptions.

2. **Polling (Scanned):**
   - *Startup Folders:* Periodically scanned. File System watchers can be expensive, so a hybrid 5-minute poll is preferred unless MiniFilter telemetry is available.
   - *WMI / Scheduled Tasks:* Polled intermittently or captured at engine startup to establish a baseline snapshot.

## Read-Only Guarantee
Under no circumstances will a `PersistenceProvider` alter system state. 
- The engine uses `KEY_READ` for registry APIs.
- The engine uses `WMI_READ_ONLY` equivalents for COM.
- Remediation (deleting malicious persistence) is strictly relegated to a future `engine-remediation`.

## Supported Mechanisms
Phase 4 implements the framework for:
- Registry Run/RunOnce
- IFEO (Image File Execution Options)
- AppInit DLLs

Stubs are provided for:
- WMI Event Subscriptions
- Scheduled Tasks
- Windows Services

## Privilege Requirements
- The engine requires `SYSTEM` context to successfully query `HKLM\SYSTEM` hives and WMI repositories. 
- If running as standard user, the engine gracefully emits `ProcessEngineError::AccessDenied` for restricted locations (e.g., HKLM) while continuing to monitor user-level persistence (e.g., HKCU).
