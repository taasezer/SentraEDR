# ADR 0029: Zero-Copy Parsing

## Status
Accepted

## Decision
All parsers inside `engine-etw` must implement the `EventRecordParser` trait utilizing zero-copy spans (`&[u8]`). Heap allocations via `Vec` or `String` clone are strictly prohibited during raw buffer traversal unless explicitly required to normalize the final output schema. This protects the agent from OOM spikes during heavy Process/Registry ETW load.
