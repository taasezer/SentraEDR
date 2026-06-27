# Phase 8 Memory Inspection Design

Date: 2026-06-27
Status: Approved for implementation by project roadmap continuation

## Goal

Phase 8 adds `engine-memory`, a safe observe-only memory telemetry analyzer. It detects suspicious memory and injection indicators from normalized metadata without reading process memory, injecting code, suspending processes, or calling Windows memory APIs.

## Approach

The engine consumes `NormalizedTelemetryEvent` records with `TelemetryAction::MemoryEventObserved`. It parses metadata emitted by future ETW/Sysmon adapters and returns preliminary `MemorySignal` values.

This phase intentionally avoids a real memory scanner. It does not call `OpenProcess`, `ReadProcessMemory`, `VirtualQueryEx`, `CreateRemoteThread`, debug APIs, minidump APIs, or kernel drivers. Those adapters require a later safety review.

## Metadata Contract

The analyzer uses these metadata keys:

- `memory.event_type`: `remote_thread_created`, `executable_private_memory`, `unsigned_module_loaded`, `section_mapping`, or `protection_changed`
- `memory.source_process_id`
- `memory.target_process_id`
- `memory.module_path`
- `memory.protection`
- `memory.region_kind`
- `memory.allocation_size`
- `memory.thread_start_address`

Missing `memory.event_type` means the event is ignored.

## Components

`engine-memory::event`

- Owns `MemoryEvent` and `MemoryEventKind`.
- Parses memory metadata from shared telemetry schemas.

`engine-memory::signal`

- Owns `MemorySignal` and `SignalSeverity`.
- Maps safe metadata indicators to observe-only signal names.

`engine-memory::analyzer`

- Owns `MemoryAnalyzer`, stats, and reports.
- Maintains no long-lived memory map in this phase.

## Initial Signals

- `remote_thread_creation`: remote thread metadata was observed.
- `executable_private_memory`: private memory region is executable.
- `unsigned_module_loaded`: loaded module metadata indicates unsigned image.
- `suspicious_section_mapping`: section mapping metadata is present.
- `memory_protection_escalation`: memory protection changed to executable permissions.

## Safety Rules

- No direct dependency on peer engine crates.
- No Windows API calls.
- No memory reads.
- No dumps.
- No code injection.
- No process suspension.
- No remediation.
- No final alerting.

## Testing

Tests cover remote thread metadata, executable private memory metadata, unsigned module metadata, section mapping metadata, protection escalation metadata, and ignored non-memory telemetry.

