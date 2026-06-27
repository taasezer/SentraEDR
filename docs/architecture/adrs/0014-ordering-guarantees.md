# ADR 0014: Ordering Guarantees

## Status
Accepted

## Decision
Ordering is **strictly guaranteed only per-producer, per-topic**. Global ordering across multiple producers or multiple topics is explicitly NOT guaranteed due to the asynchronous nature of the channels. If temporal correlation is required, consumers must rely on the embedded `Timestamp` and `CorrelationId` fields rather than the arrival time of the message.
