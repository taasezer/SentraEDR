use uuid::Uuid;

/// A globally unique identity for a persistence mechanism.
/// Must survive rescans and not rely purely on registry path (e.g. hash of path + mechanism type).
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct PersistenceIdentity {
    pub provider_type: String, // e.g., "RegistryRunKey", "ScheduledTask"
    pub location_hash: String, // Hash of the path/name
}

/// Immutable details about the persistence location.
#[derive(Debug, Clone)]
pub struct PersistenceMetadata {
    pub location_path: String,
    pub target_binary_path: String,
    pub author: Option<String>,
}

/// A dynamic modification to a persistence mechanism.
#[derive(Debug, Clone)]
pub enum PersistenceStateChange {
    Added,
    Modified {
        old_target: String,
        new_target: String,
    },
    Deleted,
}

/// A point-in-time snapshot of the persistence mechanism.
#[derive(Debug, Clone)]
pub struct PersistenceSnapshot {
    pub identity: PersistenceIdentity,
    pub metadata: PersistenceMetadata,
    pub is_active: bool,
    pub snapshot_id: Uuid,
}
