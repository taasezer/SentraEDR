# SentraEDR Performance & Metrics Strategy

## Core Philosophy
Metrics are not temporary debugging tools; they are reusable, first-class infrastructure. The system must continuously track its health to detect telemetry overload, queue saturation, and CPU spikes.

## Tracked Metrics
The `engine-etw/src/metrics.rs` module exposes global atomic counters for:

1. **Ingestion & Throughput:**
   - `events_received`: Total events hit by the C-callback.
   - `events_parsed`: Total events successfully converted to `RawEtwEvent`.
   - `events_normalized`: Total events converted to `NormalizedTelemetryEvent`.

2. **Errors & Drops:**
   - `parser_failures`: Count of events that could not be parsed.
   - `normalization_failures`: Count of events that failed schema translation.
   - `dropped_events`: Total events explicitly dropped.

3. **Queue Health:**
   - `queue_depth`: Current items in the normalization channel.
   - `queue_overflow_count`: Number of times the queue reached capacity and dropped events.

4. **Latency Tracking (Future/Sampled):**
   - `parsing_latency_us`: Moving average/histogram of time spent in the C-callback.
   - `normalization_latency_us`: Time spent converting raw events.

## Backpressure & Queue Saturation Policy
- **Maximum Queue Depth:** Internal mpsc channels (e.g., from parser to normalizer) are bounded. Default capacity is strictly limited (e.g., 10,000 events) depending on priority limits.
- **Producer Behavior:** Producers (especially the OS ETW callback thread) must NEVER block on `send()`. Under saturation, they must use non-blocking insertions (`try_send`).
- **Overflow & Drop Policy:** When a queue is saturated, we enforce a `DROP_OLDEST` (for standard telemetry) or `DROP_LOW_PRIORITY` policy. Critical security events (Process Injection, Persistence) bypass standard queues or enforce older-event-eviction to guarantee delivery.
- **Consumer Recovery:** Downstream consumers (detection engine) dynamically adjust batch processing sizes to drain saturated queues faster, temporarily suspending lower-priority correlation tasks to prioritize backlog recovery.
