use uuid::Uuid;
// We would ideally import `ProcessIdentity` from `engine-process`, but since engines must be independent, 
// they can only share models through `shared-models`.
// So we define a local proxy or expect `shared-models` to host the canonical `ProcessIdentity`.
// Since the instruction says "consume only shared models", we will assume the Detection engine joins them.
// But the user requested "ProcessIdentity" inside ConnectionIdentity. 
// We will represent the ProcessIdentity fields directly or assume it's passed from the event.

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ConnectionIdentity {
    pub process_id: u32,
    pub process_creation_time_ms: u64,
    pub local_address: String,
    pub local_port: u16,
    pub remote_address: String,
    pub remote_port: u16,
    pub protocol: String, // "TCP", "UDP"
}

#[derive(Debug, Clone)]
pub struct ConnectionMetadata {
    pub is_ipv6: bool,
    pub is_loopback: bool,
    pub is_local_subnet: bool,
}

#[derive(Debug, Clone)]
pub enum ConnectionStateChange {
    Established,
    Closed { duration_ms: u64 },
    DnsResolved { hostname: String },
}

#[derive(Debug, Clone)]
pub struct ConnectionSnapshot {
    pub identity: ConnectionIdentity,
    pub metadata: ConnectionMetadata,
    pub resolved_hostname: Option<String>,
    pub snapshot_id: Uuid,
    pub timestamp_ms: u64,
}
