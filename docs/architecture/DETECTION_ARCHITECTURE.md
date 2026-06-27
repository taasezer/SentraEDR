# Detection Engine Architecture

The Detection Engine correlates discrete telemetry events into actionable, highly explainable `Alert` objects. It is the ONLY component in SentraEDR authorized to generate security alerts.

## 6-Stage Pipeline

1. **Intake:** Normalizes upstream engine snapshots and events (e.g. `ProcessSnapshot`, `ConnectionSnapshot`) into a unified `CorrelationState`.
2. **Correlation:** State retention layer. Events are indexed by `ProcessIdentity`, `Timestamp`, and `EventId`.
3. **Rule Evaluation:** Pluggable rules evaluate the current `CorrelationState`.
4. **Risk Scoring:** The severity of the impact.
5. **Confidence Evaluation:** The certainty that the behavior is malicious. Risk and Confidence are strictly decoupled.
6. **Alert Generation:** Synthesizing the final immutable `Alert`.

## Memory Budgeting & Correlation Lifecycle

The `CorrelationState` is explicitly bounded to prevent unbounded memory growth.
- **Rule-Driven Expiration:** There is no global correlation window. Each `Rule` dictates its `max_correlation_window_ms()`. The engine tracks the maximum window required by all active rules, and aggressively purges any event older than that threshold.
- **Max Limits:** The engine enforces a hard limit on the total number of events (e.g., 50,000) and max events per correlation chain (e.g., 100) to defend against memory exhaustion during event floods.
- **Cleanup Policy:** A periodic sweep executes evictions based on the TTLs.

## Evidence & Explainability
Alerts must be mathematically reproducible.
- Large event payloads (e.g. 2MB registry values) are NOT duplicated into the Alert structure unless explicitly mandated by the rule.
- Instead, the Alert references immutable `EventId`s.
- The `reasoning_path` field clearly describes the logical steps the rule took to synthesize the alert.

## External Rule Packs
Rules implement a generic `Rule` trait, allowing the future introduction of external YAML/JSON rule parsers without recompiling the detection engine core.
