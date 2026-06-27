use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProcessEngineError {
    #[error("Access Denied querying OS resource for PID {0}")]
    AccessDenied(u32),
    #[error("Process {0} not found (terminated before query)")]
    ProcessNotFound(u32),
    #[error("OS API Failure: {0}")]
    OsApiFailure(String),
}
