# Process Monitoring Engine Architecture

The Process Monitoring Engine (`engine-process`) strictly observes and enriches telemetry. It does not perform threat detection or behavioral correlation.

## Engine Contract
Like all engines in SentraEDR, this module strictly follows the standardized internal layout:
- `source.rs`: Abstractions over Windows APIs (`OpenProcess`, `GetTokenInformation`).
- `analyzer.rs`: The state machine and caching logic for enrichment.
- `models.rs`: Enrichment structs (`ProcessIdentity`, `ProcessMetadata`, etc.).
- `metrics.rs`: Performance tracking.
- `errors.rs`: Structured engine failures.
- `tests/`: Isolated verification.

## Event Immutability
`NormalizedTelemetryEvent` objects consumed by the engine are fully immutable. The engine may only read these events and produce *new* enrichment structures (`ProcessSnapshot`, `ProcessStateChange`) that wrap or reference the original identity.

## Separation of State
To manage memory safely during long uptimes, process information is explicitly classified:

1. **ProcessMetadata (Immutable Attributes):**
   - Command Line
   - Image Path
   - Session ID
   - Initial User SID
   - These are collected *once* upon process creation and cached.

2. **ProcessStateChange (Transitions):**
   - A modification to the process that occurs during its lifetime (e.g., token elevation, thread injection).
   - These bypass the static cache and are emitted directly as discrete state changes.

3. **ProcessSnapshot (Point-in-Time):**
   - The merged output combining the static `ProcessMetadata` (from the cache) and the dynamic state implied by the current `NormalizedTelemetryEvent`.
   - This is what the Detection Engine consumes.

## Handle Management (RAII)
All Windows `HANDLE`s are wrapped in strict RAII structs. 
- Ownership is explicit.
- `CloseHandle` is invoked automatically via the `Drop` trait.
- A raw `HANDLE` never escapes the `source.rs` module.

## Privilege Awareness
EDR agents run as `SYSTEM`, but PPL (Protected Process Light) processes (e.g., `csrss.exe`, `smss.exe`, third-party AV) will return `ACCESS_DENIED` upon `OpenProcess`. 
- The engine does NOT treat `ACCESS_DENIED` as a panic or failure.
- It produces a structured error, falls back to the cache (if the process was logged by ETW), and emits a partial `ProcessSnapshot` tagged with limited visibility flags.
