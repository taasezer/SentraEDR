pub mod metrics;
pub mod provider;
pub mod session;
pub mod parser;
pub mod normalizer;

// The ETW pipeline interface will be built here, connecting the OS thread
// running `ProcessTrace` with the Tokio async channel reading `RawEtwEvent` 
// into `normalizer::normalize()`.
