# SentraEDR Engineering Discipline

## Overview
This document defines the strict engineering, validation, and operational practices that govern the SentraEDR project. Adherence is mandatory to maintain a production-grade platform.

For architectural layout, see [ARCHITECTURE.md](ARCHITECTURE.md).
For security principles, see [SECURITY_MODEL.md](../security/SECURITY_MODEL.md).

## 1. Repository Discipline
- **Strict Folder Organization:** The codebase is split into `/engine`, `/shared`, `/ui`, `/tests`, `/benchmarks`, and `/docs`.
- **Crate Responsibility Boundaries:** Each crate must have a single responsibility. Cross-dependencies between engines are forbidden. All engines communicate solely through `shared-ipc` using schemas from `shared-models`.
- **Deterministic Configuration:** Configurations must be reproducible, and dependencies strictly version-locked.

## 2. Incremental Validation Workflow
- **Continuous Validation:** Every subsystem must be validated theoretically, structurally (architecture), and practically (memory, performance) before, during, and after implementation.
- **Dependency checks:** The dependency graph must remain acyclic. No direct engine-to-engine calls are permitted.

## 3. Telemetry Benchmarking Strategy
- **Continuous Measurement:** ETW throughput, IPC latency, queue depth, and memory allocations must be benchmarked regularly.
- **Stress Testing:** The system must be tested under idle load, high event load, and burst scenarios (e.g., registry spam).

## 4. ETW Debugging Methodology
- **Traceability:** Provider configurations, event schemas, and parsing assumptions must be documented.
- **Lag Monitoring:** The ingestion pipeline must monitor and report on event drops or consumer lag, falling back gracefully under pressure. See [TELEMETRY_MODEL.md](../telemetry/TELEMETRY_MODEL.md) for specifics.

## 5. Windows Internals Considerations
- **Validation:** All low-level assumptions (WinAPI behavior, thread scheduling, memory permissions) must be validated against NT internals.
- **Privilege Awareness:** The engine must account for token privileges, handle inheritance, and WOW64 behavior without making unsafe undocumented assumptions.

## 6. Human-in-the-Loop Development Workflow
- The AI operates as a collaborative team. Every phase concludes with a mandatory stop. The human user validates the phase, executes terminal commands, and approves continuation.
- The AI will not autonomously run terminal commands that modify system state without explicit human approval.

## 7. Critical Failure Modes (Guardrails)
The system is explicitly designed to prevent three critical failure classes:

### A. Telemetry Overload
- **Risk:** Event storms crash the agent or consume excessive RAM.
- **Mitigation:** Strict bounded queues, priority-based event dropping (low-priority events drop first), and adaptive throttling.

### B. Wrong Remediation
- **Risk:** The EDR breaks the OS by terminating critical processes or deleting safe files.
- **Mitigation:** Quarantine-first policy. Two-step validation required before remediation. High-confidence scoring (>=85) and multi-signal agreement are mandatory. See [DETECTION_ENGINE.md](DETECTION_ENGINE.md).

### C. Crate Coupling Breakdown
- **Risk:** Loss of modular isolation leads to circular dependencies and unpredictable state.
- **Mitigation:** One-way data flow enforced at the build level. `shared-ipc` is the only bridge. `shared-models` is the immutable contract layer.
