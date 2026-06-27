# ADR 0023: Clock Abstraction

## Status
Accepted

## Decision
Direct calls to `std::time::SystemTime` are prohibited outside of the infrastructure layer. The platform relies on a `Clock` trait. This guarantees that temporal tests (like TTL expirations in the Detection Engine) can be deterministically mocked.
