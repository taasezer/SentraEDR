use thiserror::Error;

#[derive(Debug, Error)]
pub enum PersistenceEngineError {
    #[error("Access Denied reading persistence location: {0}")]
    AccessDenied(String),
    #[error("Persistence Key Not Found: {0}")]
    KeyNotFound(String),
    #[error("Provider Failure ({0}): {1}")]
    ProviderFailure(String, String),
}
