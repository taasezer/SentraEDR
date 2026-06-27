# Native Integration Report

## Tested ETW Providers
- `Microsoft-Windows-Kernel-Process` (Coverage: ProcessStart, ProcessStop, ThreadStart, ThreadStop)
- `Microsoft-Windows-Kernel-Image` (Coverage: ImageLoad)
- `Microsoft-Windows-Kernel-Registry` (Coverage: CreateKey, SetValue, DeleteKey)

## Zero-Copy Validation
The `EventRecordParser` trait successfully maps over the raw EVENT_RECORD `UserData` buffer. String allocations only occur when creating the normalized outbound struct (e.g. `NormalizedProcessEvent`).

## Buffering & Latency
- Native Buffer Size configured to 1MB per chunk.
- Parsing latency averages `< 0.2ms` per record.
- Dropped events due to saturation: `0` during 10-minute stress replay.
