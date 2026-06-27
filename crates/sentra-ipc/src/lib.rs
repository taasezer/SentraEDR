//! # sentra-ipc
//!
//! Inter-component communication primitives for the **SentraEDR** platform.
//!
//! This crate provides the plumbing that connects every subsystem — telemetry
//! collection, detection engine, remediation, and the service shell — through
//! bounded, backpressure-aware channels with built-in health monitoring.
//!
//! ## Modules
//!
//! | Module | Purpose |
//! |---|---|
//! | [`bus`] | Broadcast-based event bus for fan-out telemetry delivery |
//! | [`channel`] | Bounded mpsc channels with health counters |
//! | [`priority`] | Multi-level priority queue with automatic shedding |
//! | [`messages`] | Typed IPC message envelope |
//!
//! ## Design principles
//!
//! - **All channels are bounded** — no unbounded queues anywhere.
//! - **No `unwrap()`** — every fallible path uses `?` or explicit handling.
//! - **Lock-free fast path** — atomic counters for metrics, `broadcast` for
//!   fan-out, `mpsc` for point-to-point.
//! - **Pressure shedding** — the priority queue drops lower-priority items
//!   before higher ones when capacity is exhausted.

pub mod bus;
pub mod channel;
pub mod messages;
pub mod priority;
pub mod pipe;

// ── Convenience re-exports ──────────────────────────────────────────────

pub use bus::{BusHealth, EventBus};
pub use channel::{BoundedChannel, ChannelHealth, ChannelReceiver, ChannelSender};
pub use messages::IpcMessage;
pub use priority::{PriorityDropCounts, PriorityQueue};
pub use pipe::*;
