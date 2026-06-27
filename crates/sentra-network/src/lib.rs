//! # sentra-network
//!
//! Network connection monitoring, suspicious connection detection, and C2
//! beaconing analysis for the **SentraEDR** platform on Windows.
//!
//! ## Modules
//!
//! | Module          | Purpose |
//! |-----------------|---------|
//! | [`connections`] | Enumerates active TCP/UDP connections via `iphlpapi` |
//! | [`anomaly`]     | Detects suspicious ports, beaconing, and connection storms |
//! | [`dns`]         | DNS cache tracking, DGA detection, and tunneling analysis |
//!
//! ## Quick start
//!
//! ```no_run
//! use sentra_network::NetworkMonitor;
//!
//! #[tokio::main]
//! async fn main() {
//!     let monitor = NetworkMonitor::new(Default::default());
//!     let snapshot = monitor.snapshot().expect("enumeration failed");
//!     let detections = monitor.analyze(&snapshot);
//!     for d in &detections {
//!         println!("[{}] {}", d.rule_name, d.description);
//!     }
//! }
//! ```

pub mod anomaly;
pub mod connections;
pub mod dns;

pub use anomaly::{NetworkAnomalyDetector, TimestampedConnection};
pub use connections::{enumerate_tcp_connections, enumerate_udp_endpoints, get_all_connections};
pub use dns::DnsCache;

use parking_lot::RwLock;
use sentra_core::{DetectionResult, NetworkConnection, SentraError};
use std::sync::Arc;

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

/// Tunable parameters for the [`NetworkMonitor`].
#[derive(Debug, Clone)]
pub struct NetworkMonitorConfig {
    /// Maximum entries in the DNS cache before LRU eviction.
    pub dns_cache_size: usize,
    /// Minimum connections before beaconing analysis fires.
    pub beaconing_min_connections: u64,
    /// Maximum jitter ratio (0.0–1.0) for beaconing detection.
    pub beaconing_max_jitter: f64,
    /// Connection-rate threshold (connections per snapshot) for storms.
    pub connection_rate_threshold: usize,
}

impl Default for NetworkMonitorConfig {
    fn default() -> Self {
        Self {
            dns_cache_size: 4096,
            beaconing_min_connections: 5,
            beaconing_max_jitter: 0.15,
            connection_rate_threshold: 100,
        }
    }
}

// ---------------------------------------------------------------------------
// NetworkMonitor
// ---------------------------------------------------------------------------

/// Central façade that orchestrates connection enumeration, anomaly
/// detection, and DNS analysis.
///
/// Thread-safe: the monitor is `Send + Sync` and can be shared across
/// async tasks via `Arc`.
pub struct NetworkMonitor {
    /// Configuration knobs.
    config: NetworkMonitorConfig,
    /// The anomaly detector instance.
    detector: NetworkAnomalyDetector,
    /// DNS query cache with DGA / tunneling analysis.
    dns_cache: Arc<RwLock<DnsCache>>,
}

impl NetworkMonitor {
    /// Create a new monitor with the given configuration.
    pub fn new(config: NetworkMonitorConfig) -> Self {
        let detector = NetworkAnomalyDetector::new(
            config.beaconing_min_connections,
            config.beaconing_max_jitter,
        );
        let dns_cache = Arc::new(RwLock::new(DnsCache::new(config.dns_cache_size)));
        Self {
            config,
            detector,
            dns_cache,
        }
    }

    /// Take a point-in-time snapshot of all TCP and UDP connections.
    pub fn snapshot(&self) -> sentra_core::Result<Vec<NetworkConnection>> {
        get_all_connections()
    }

    /// Run every anomaly detector against the provided connection list and
    /// return all detections (may be empty).
    pub fn analyze(&self, connections: &[NetworkConnection]) -> Vec<DetectionResult> {
        let mut results = Vec::new();

        // Per-connection checks
        for conn in connections {
            if let Some(det) = self.detector.detect_suspicious_ports(conn) {
                results.push(det);
            }
            if let Some(det) = self.detector.detect_unusual_outbound(conn) {
                results.push(det);
            }
        }

        // Aggregate checks
        let storm_dets =
            NetworkAnomalyDetector::detect_high_connection_rate(connections, self.config.connection_rate_threshold);
        results.extend(storm_dets);

        // DNS tunneling
        {
            let cache = self.dns_cache.read();
            let dns_dets = cache.detect_tunneling();
            results.extend(dns_dets);
        }

        results
    }

    /// Record a DNS query into the internal cache for later analysis.
    pub fn record_dns_query(&self, query: sentra_core::DnsQueryInfo) {
        let mut cache = self.dns_cache.write();
        cache.record_query(query);
    }

    /// Return `true` if the given domain looks like it was produced by a
    /// domain generation algorithm (DGA).
    pub fn is_dga_domain(domain: &str) -> bool {
        DnsCache::detect_dga(domain)
    }

    /// Obtain a reference-counted handle to the DNS cache.
    pub fn dns_cache(&self) -> Arc<RwLock<DnsCache>> {
        Arc::clone(&self.dns_cache)
    }

    /// Return a reference to the active configuration.
    pub fn config(&self) -> &NetworkMonitorConfig {
        &self.config
    }
}

/// Convert a Win32 error code into a [`SentraError::WindowsApi`].
pub(crate) fn win32_error(code: u32, context: &str) -> SentraError {
    SentraError::WindowsApi(format!("{context}: Win32 error {code:#010x}"))
}
