# ADR 0016: Subscriber Isolation

## Status
Accepted

## Decision
Subscribers must never impact one another. Using `tokio::sync::broadcast`, each subscriber maintains its own independent cursor into the ring buffer. If Subscriber A is slow and falls behind, the broadcast channel will eventually overwrite the oldest messages. Subscriber A will receive a `Lagged` error, but Subscriber B (who is reading quickly) will experience zero latency or disruption.
