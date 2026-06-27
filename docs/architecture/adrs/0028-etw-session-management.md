# ADR 0028: ETW Session Management

## Status
Accepted

## Decision
The `EtwSession` must run inside its own dedicated OS thread using `std::thread::spawn` instead of `tokio::spawn`. The Windows API `ProcessTrace` is a blocking C call that will permanently starve a Tokio worker thread if executed inside the async reactor. The ETW thread will parse events and push them onto the EventBus asynchronously.
