# SentraEDR Memory Model

## Target Footprint
**Goal:** < 150MB idle RAM usage.

## Allocation Strategy
- **Zero-Copy Where Possible:** Event ingestion and processing should utilize references and slices instead of cloning strings.
- **Pre-allocation:** Buffers for ETW events and network packets must be pre-allocated and reused via pooling where applicable.
- **Bounded Queues:** All internal communication channels (e.g., mpsc channels) must be strictly bounded to prevent memory explosion during event storms.

### Hot Path Allocation Rules
The ETW `parser` and `normalizer` form the system's absolute hot path. 
1. **Dynamic Allocations:** Must be avoided. We do not use `serde_json` or heavy heap-allocated objects for intermediate representations.
2. **Cloning:** We clone string data (e.g., process paths) exactly once when moving from the C-managed `EVENT_RECORD` buffer into the owned `RawEtwEvent`. This is mandatory because the ETW buffer pointer becomes invalid immediately after the C-callback returns. No further cloning of this string is permitted downstream; we pass it by ownership or `Arc` if necessary.

## Serialization
- Use efficient binary formats (like `bincode`) for IPC rather than JSON in the hot path.
- Keep `shared-models` structs lightweight.

## Eviction Policies
- State caches (e.g., process tracking, network connection tracking) must implement aggressive LRU (Least Recently Used) or TTL (Time To Live) eviction to ensure stale data is purged automatically.
- Under severe memory pressure, the system degrades gracefully by dropping low-priority telemetry and temporarily pausing non-critical analysis components.
