use std::collections::HashMap;
use shared_models::events::{NormalizedTelemetryEvent, EventType};
use crate::models::{ConnectionIdentity, ConnectionMetadata, ConnectionSnapshot, ConnectionStateChange};
use crate::metrics::METRICS;
use uuid::Uuid;

/// Lightweight LRU Cache mapping IP addresses to resolved DNS hostnames.
/// Operates as an optimization layer only. 
pub struct DnsCache {
    // In production, this would use a proper LRU crate bounded to ~10,000 entries.
    entries: HashMap<String, String>,
}

impl DnsCache {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn insert(&mut self, ip: String, hostname: String) {
        self.entries.insert(ip, hostname);
    }

    pub fn lookup(&self, ip: &str) -> Option<&String> {
        self.entries.get(ip)
    }
}

/// The core network correlation loop.
pub struct NetworkAnalyzer {
    dns_cache: DnsCache,
}

impl NetworkAnalyzer {
    pub fn new() -> Self {
        Self {
            dns_cache: DnsCache::new(),
        }
    }

    /// Non-blocking event correlation. Consumes normalized network telemetry.
    pub fn process_event(&mut self, event: &NormalizedTelemetryEvent) -> Option<(ConnectionSnapshot, ConnectionStateChange)> {
        match &event.event_type {
            EventType::NetworkConnection { destination_ip, destination_port, protocol } => {
                let identity = ConnectionIdentity {
                    process_id: event.process_id,
                    process_creation_time_ms: 0, // Would pull from a shared process registry or enriched event
                    local_address: "0.0.0.0".to_string(),
                    local_port: 0,
                    remote_address: destination_ip.clone(),
                    remote_port: *destination_port,
                    protocol: protocol.clone(),
                };

                let metadata = ConnectionMetadata {
                    is_ipv6: destination_ip.contains(':'),
                    is_loopback: destination_ip.starts_with("127.") || destination_ip == "::1",
                    is_local_subnet: false, // Simplified
                };

                // Non-blocking DNS Cache Lookup
                let hostname = if let Some(host) = self.dns_cache.lookup(destination_ip) {
                    METRICS.inc_cache_hit();
                    Some(host.clone())
                } else {
                    METRICS.inc_cache_miss();
                    // In a real implementation, we would spawn an async resolution task here using Tokio.
                    // If it fails, we do not block. We return None for the hostname.
                    None
                };

                let snapshot = ConnectionSnapshot {
                    identity,
                    metadata,
                    resolved_hostname: hostname,
                    snapshot_id: Uuid::new_v4(),
                    timestamp_ms: event.timestamp_ms,
                };

                Some((snapshot, ConnectionStateChange::Established))
            }
            _ => None,
        }
    }
}
