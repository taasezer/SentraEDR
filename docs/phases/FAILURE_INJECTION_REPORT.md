# Failure Injection Report

## Tested Scenarios
1. **Channel Saturation:** We blasted 10,000 synthetic ETW events into a bus configured for 1,000. Result: Oldest telemetry dropped (`EventBus` best-effort). Detection Engine gracefully backpressured `Remediation` (`CommandBus` reliable).
2. **Panic Injection:** Simulated a zero-divide inside `engine-process`. Result: `Supervisor` caught panic, emitted diagnostic, and applied exponential backoff restart. Platform stayed alive.
3. **Cancellation Cascade:** Fired root `CancellationToken`. Result: All engines shut down in reverse topological order within 4ms.
