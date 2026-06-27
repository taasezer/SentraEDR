# Architecture Overview

SentraEDR comprises 21 cleanly separated Rust crates spanning four domains:
1. **Infrastructure:** Abstracted `CommandBus`, `EventBus`, and Storage providers.
2. **Engines:** Highly concurrent, zero-copy ETW telemetry consumers, Detection Rule runners, and Typestate-enforced Remediation logic.
3. **Core Platform:** The `RuntimeBuilder` and `Supervisor`, acting as the composition root handling lifecycles, health metrics, and failure recovery.
4. **UI Client:** A strictly decoupled Tauri frontend enabling isolated local administration.
