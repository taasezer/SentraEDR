# Network Monitoring Architecture

The Network Monitoring Engine (`engine-network`) is an observation-only component. It correlates low-level connection metadata with process identities and enriches IPs with DNS hostnames.

## Transport Independence
The engine is completely decoupled from the transport layer. It operates exclusively against the `NetworkProvider` trait. 

## DNS Enrichment Policy
DNS resolution is fundamentally asynchronous and must NEVER block the main telemetry ingestion pipeline.
- **Cache Definition:** The engine maintains an internal `DnsCache`.
- **Max Size:** Bounded (e.g., 10,000 entries) to prevent memory exhaustion during port scans or heavy traffic.
- **Expiration Policy:** Entries TTL out after a set time (e.g., 1 hour) to account for dynamic IP reassignment (CDNs).
- **Eviction Policy:** Strict LRU (Least Recently Used) when the max size is hit.
- **Failure Policy:** If a DNS lookup fails or times out, the engine emits the `ConnectionSnapshot` immediately with a missing hostname rather than delaying the event.

## Event Semantics
- **ConnectionMetadata:** Immutable attributes of a network stream (IPv4 vs IPv6, TCP vs UDP).
- **ConnectionStateChange:** TCP state transitions (SYN, ESTABLISHED, CLOSED).
- **ConnectionSnapshot:** Point-in-time representation merging process identity, L3/L4 metadata, and the current DNS cache state.

## Privileges & Limitations
- The engine operates purely on metadata (Headers/IPs/Ports). L7 payload inspection (TLS decryption) is explicitly out of scope for this phase.
- No network connections will be blocked or modified.
