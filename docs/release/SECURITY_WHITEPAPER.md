# Security Whitepaper

## The Zero-Trust Agent Architecture
SentraEDR employs a fundamentally zero-trust approach to its own architecture.

- **Rule Sandboxing:** Detection rules are loaded into restricted scopes. They cannot access the filesystem, registry, or network.
- **Client Isolation:** The UI is physically and logically separated from the telemetry pipeline.
- **Memory Safety:** Built 100% in Rust, entire classes of memory vulnerabilities (Buffer Overflows, Use-After-Free) are eliminated at compile time.
- **Data Integrity:** Persistence relies on strict atomic transactions. No global mutable state exists.
