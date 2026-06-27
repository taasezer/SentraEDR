use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetworkEngineError {
    #[error("Provider Failure: {0}")]
    ProviderFailure(String),
    #[error("DNS Resolution Timeout for {0}")]
    DnsTimeout(String),
    #[error("Unknown Protocol ID: {0}")]
    UnknownProtocol(u8),
}
