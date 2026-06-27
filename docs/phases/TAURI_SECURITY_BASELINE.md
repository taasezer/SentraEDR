# Tauri Security Baseline

## Isolation Configuration
- Isolation pattern enabled (`"pattern": "isolation"`). This routes all IPC messages through a secure iframe boundary, preventing malicious frontend code from sniffing or modifying IPC payloads.

## Capability Minimization
- Shell access completely disabled (`"shell": { "all": false }`). The UI has zero ability to spawn `cmd.exe` or `powershell.exe`.
- Filesystem access disabled (`"fs": { "all": false }`). The UI cannot arbitrarily read or write files to the disk. All persistence occurs over the `CommandBus`.
