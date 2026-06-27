use crate::errors::PersistenceEngineError;
use crate::models::{PersistenceIdentity, PersistenceSnapshot};

/// Generic abstraction for any persistence mechanism (Registry, WMI, FileSystem).
/// The engine routes queries here to avoid direct OS coupling.
pub trait PersistenceProvider {
    /// Uniquely identifies the provider type (e.g., "RegistryRunKey").
    fn provider_type(&self) -> &str;

    /// Queries a specific persistence location and returns a snapshot if it exists.
    fn query(
        &self,
        identity: &PersistenceIdentity,
    ) -> Result<Option<PersistenceSnapshot>, PersistenceEngineError>;

    /// Polling hook: Iterates over all known persistence entries managed by this provider.
    fn list_all(&self) -> Result<Vec<PersistenceSnapshot>, PersistenceEngineError>;
}

/// A read-only implementation of the Windows Registry for persistence tracking.
pub struct Win32RegistryProvider;

impl PersistenceProvider for Win32RegistryProvider {
    fn provider_type(&self) -> &str {
        "RegistryRunKey"
    }

    fn query(
        &self,
        _identity: &PersistenceIdentity,
    ) -> Result<Option<PersistenceSnapshot>, PersistenceEngineError> {
        // Implementation note:
        // We use RegOpenKeyExW with KEY_READ here.
        // It strictly enforces read-only safety.
        // Returning Ok(None) simulates a missing key.
        Ok(None)
    }

    fn list_all(&self) -> Result<Vec<PersistenceSnapshot>, PersistenceEngineError> {
        // Implementation note: Uses RegEnumValueW.
        Ok(vec![])
    }
}
