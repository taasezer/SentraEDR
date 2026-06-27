# VM Testing Setup Guide

## Requirements
- Windows 11 Enterprise (22H2+) VM.
- 16GB RAM, 4 vCPUs.
- Elevated Command Prompt (Administrator) or NT AUTHORITY\SYSTEM context for ETW session control.

## Execution
Run the following to commence the 24-hour native soak test, validating absolute memory boundaries:
`cargo xtask soak --duration=24h`
