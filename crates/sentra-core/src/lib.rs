//! # sentra-core
//!
//! Foundation crate for the **SentraEDR** Anti-RAT / EDR platform.
//!
//! This crate provides the shared vocabulary used by every other crate
//! in the workspace:
//!
//! | Module          | Purpose |
//! |-----------------|---------|
//! | [`error`]       | Unified error type and `Result` alias |
//! | [`types`]       | Domain types (events, detections, etc.) |
//! | [`traits`]      | Pipeline integration traits |
//! | [`config`]      | TOML-based hierarchical configuration |
//!
//! No Windows API calls are made here — this crate is pure Rust and
//! could (in principle) be compiled on any target for testing.

/// Unified error types for the SentraEDR platform.
pub mod error;
pub mod types;
pub mod traits;
pub mod config;

pub use error::*;
pub use types::*;
pub use traits::*;
pub use config::*;
