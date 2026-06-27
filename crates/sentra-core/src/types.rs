//! Shared domain types for the SentraEDR platform.
//!
//! This module defines every data structure that flows through the EDR
//! pipeline — from raw telemetry events emitted by sensors, through
//! the detection engine, to remediation actions and health reports.
//!
//! All types are `Send + Sync` and derive [`serde::Serialize`] /
//! [`serde::Deserialize`] so they can be transmitted over IPC channels
//! and persisted to disk.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Telemetry Events
// ---------------------------------------------------------------------------

/// Top-level telemetry event emitted by any sensor in the system.
///
/// Each variant wraps a typed payload struct that carries all the
/// context needed by downstream detection rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TelemetryEvent {
    /// A new process was created.
    ProcessCreate(ProcessInfo),
    /// A process terminated.
    ProcessTerminate(ProcessInfo),
    /// An outbound or inbound network connection was established.
    NetworkConnect(NetworkConnection),
    /// A network connection was torn down.
    NetworkDisconnect(NetworkConnection),
    /// A registry key or value was modified.
    RegistryModify(RegistryEvent),
    /// A new file was created on disk.
    FileCreate(FileEvent),
    /// An existing file was modified.
    FileModify(FileEvent),
    /// A PowerShell script block or command was executed.
    PowerShellExec(PowerShellEvent),
    /// A DLL was loaded into a process address space.
    DllLoad(DllLoadInfo),
    /// A Windows service was installed, started, stopped, or changed.
    ServiceChange(ServiceInfo),
    /// An unstructured / raw event from an ETW provider.
    RawInput(RawInputEvent),
    /// A DNS query was issued.
    DnsQuery(DnsQueryInfo),
}

// ---------------------------------------------------------------------------
// Process
// ---------------------------------------------------------------------------

/// Windows process integrity level.
///
/// Ordered from least-privileged ([`Untrusted`](Self::Untrusted)) to
/// most-privileged ([`Protected`](Self::Protected)).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IntegrityLevel {
    /// Untrusted — lowest integrity.
    Untrusted,
    /// Low integrity (e.g., sandboxed browser tabs).
    Low,
    /// Medium integrity — standard user processes.
    Medium,
    /// Medium-plus (elevated standard user).
    MediumPlus,
    /// High integrity — administrator processes.
    High,
    /// SYSTEM integrity.
    System,
    /// Protected process (anti-malware light).
    Protected,
}

/// Snapshot of a Windows process relevant to EDR analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    /// Process ID.
    pub pid: u32,
    /// Parent process ID.
    pub ppid: u32,
    /// Executable base name (e.g., `"cmd.exe"`).
    pub name: String,
    /// Full path to the executable image.
    pub exe_path: String,
    /// Command-line string the process was started with.
    pub cmdline: String,
    /// User account running the process (e.g., `"NT AUTHORITY\SYSTEM"`).
    pub user: String,
    /// Token integrity level.
    pub integrity_level: IntegrityLevel,
    /// Timestamp when the process was started.
    pub start_time: DateTime<Utc>,
    /// Terminal-services session ID.
    pub session_id: u32,
}

// ---------------------------------------------------------------------------
// Network
// ---------------------------------------------------------------------------

/// Transport-layer protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Protocol {
    /// Transmission Control Protocol.
    Tcp,
    /// User Datagram Protocol.
    Udp,
}

/// TCP connection state (mirrors the RFC 793 state machine).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TcpState {
    /// Listening for incoming connections.
    Listen,
    /// Connection fully established.
    Established,
    /// Waiting for enough time to pass to be sure the remote peer
    /// received the acknowledgment of its connection termination request.
    TimeWait,
    /// Waiting for a connection termination request from the local user.
    CloseWait,
    /// SYN sent, waiting for SYN-ACK.
    SynSent,
    /// SYN received, SYN-ACK sent.
    SynReceived,
    /// FIN sent, waiting for ACK or FIN.
    FinWait1,
    /// FIN acknowledged, waiting for remote FIN.
    FinWait2,
    /// Both sides sent FIN simultaneously.
    Closing,
    /// Waiting for final ACK after sending FIN.
    LastAck,
    /// Fully closed.
    Closed,
}

/// A network connection observed on the host.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConnection {
    /// PID of the owning process.
    pub pid: u32,
    /// Transport protocol.
    pub protocol: Protocol,
    /// Local socket address (IP + port).
    pub local_addr: SocketAddr,
    /// Remote socket address (absent for UDP listeners / unconnected).
    pub remote_addr: Option<SocketAddr>,
    /// TCP state (meaningful only for [`Protocol::Tcp`]).
    pub state: TcpState,
    /// Name of the owning process.
    pub process_name: String,
}

// ---------------------------------------------------------------------------
// Registry
// ---------------------------------------------------------------------------

/// The type of registry operation that was observed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegistryOp {
    /// A new key or value was created.
    Create,
    /// An existing value was modified.
    Modify,
    /// A key or value was deleted.
    Delete,
    /// A key or value was renamed.
    Rename,
}

/// A registry modification event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryEvent {
    /// Full registry key path (e.g., `HKLM\SOFTWARE\...`).
    pub key_path: String,
    /// Registry value name.
    pub value_name: String,
    /// What was done.
    pub operation: RegistryOp,
    /// String representation of the new data, if applicable.
    pub data: Option<String>,
    /// PID of the process that performed the operation.
    pub pid: u32,
    /// Name of the process that performed the operation.
    pub process_name: String,
    /// When the operation occurred.
    pub timestamp: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// File
// ---------------------------------------------------------------------------

/// A file-system event (creation or modification).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEvent {
    /// Full path to the affected file.
    pub path: String,
    /// PID of the process that performed the operation.
    pub pid: u32,
    /// Name of the process that performed the operation.
    pub process_name: String,
    /// When the event occurred.
    pub timestamp: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// DLL
// ---------------------------------------------------------------------------

/// Information about a DLL loaded into a process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DllLoadInfo {
    /// PID of the host process.
    pub pid: u32,
    /// Name of the host process.
    pub process_name: String,
    /// Full path to the DLL on disk.
    pub dll_path: String,
    /// Base name of the DLL (e.g., `"ntdll.dll"`).
    pub dll_name: String,
    /// Virtual base address where the DLL was mapped.
    pub base_address: u64,
    /// Size of the mapped image in bytes.
    pub size: u64,
    /// Whether the DLL has a valid Authenticode signature.
    pub is_signed: bool,
}

// ---------------------------------------------------------------------------
// Service
// ---------------------------------------------------------------------------

/// Windows service start type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServiceStartType {
    /// Automatically started by the Service Control Manager at boot.
    Auto,
    /// Started manually via `sc start` / `StartService`.
    Manual,
    /// Service is disabled and cannot be started.
    Disabled,
    /// Loaded by the boot loader.
    Boot,
    /// Started by the I/O subsystem during kernel initialisation.
    System,
}

/// Current runtime status of a Windows service.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServiceStatus {
    /// Service is running.
    Running,
    /// Service is stopped.
    Stopped,
    /// Service is paused.
    Paused,
    /// Service is starting.
    StartPending,
    /// Service is stopping.
    StopPending,
    /// Service is pausing.
    PausePending,
    /// Service is resuming from a paused state.
    ContinuePending,
}

/// Snapshot of a Windows service relevant to persistence monitoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    /// Internal service name (used by SCM).
    pub name: String,
    /// Human-readable display name.
    pub display_name: String,
    /// Path to the service binary.
    pub binary_path: String,
    /// How the service is configured to start.
    pub start_type: ServiceStartType,
    /// Current runtime status.
    pub status: ServiceStatus,
    /// PID of the service process, if running.
    pub pid: Option<u32>,
}

// ---------------------------------------------------------------------------
// PowerShell
// ---------------------------------------------------------------------------

/// A PowerShell execution event captured via ETW or script-block logging.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerShellEvent {
    /// PID of the PowerShell host process.
    pub pid: u32,
    /// The deobfuscated script-block text.
    pub script_block: String,
    /// Original command-line string.
    pub command_line: String,
    /// Whether the original invocation used `-EncodedCommand`.
    pub is_encoded: bool,
}

// ---------------------------------------------------------------------------
// DNS
// ---------------------------------------------------------------------------

/// A DNS query issued by a process on the host.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsQueryInfo {
    /// PID of the querying process.
    pub pid: u32,
    /// Queried domain name.
    pub query_name: String,
    /// DNS record type (e.g., `"A"`, `"AAAA"`, `"CNAME"`).
    pub query_type: String,
    /// Resolved response, if available.
    pub response: Option<String>,
}

// ---------------------------------------------------------------------------
// Raw / unstructured input
// ---------------------------------------------------------------------------

/// An unstructured event that doesn't map to a specific typed variant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawInputEvent {
    /// Originating source / provider name.
    pub source: String,
    /// Opaque payload as a UTF-8 string.
    pub payload: String,
    /// When the event was captured.
    pub timestamp: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Threat & Detection
// ---------------------------------------------------------------------------

/// Severity level assigned to a detection.
///
/// Implements [`Ord`] so that levels can be compared directly
/// (`ThreatLevel::Critical > ThreatLevel::Low`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum ThreatLevel {
    /// No threat detected.
    None,
    /// Informational / low-severity indicator.
    Low,
    /// Medium-severity — warrants investigation.
    Medium,
    /// High-severity — likely malicious activity.
    High,
    /// Critical — active exploitation in progress.
    Critical,
}

/// A single piece of supporting evidence attached to a [`DetectionResult`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    /// Human-readable source description (e.g., `"process_monitor"`).
    pub source: String,
    /// Free-text detail explaining what was observed.
    pub detail: String,
    /// When the evidence was collected.
    pub timestamp: DateTime<Utc>,
}

/// Output of a detection rule after analysing a [`TelemetryEvent`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionResult {
    /// Unique detection identifier.
    pub id: Uuid,
    /// Name of the rule that fired.
    pub rule_name: String,
    /// Assigned threat severity.
    pub threat_level: ThreatLevel,
    /// Confidence score in the range `[0.0, 1.0]`.
    pub confidence: f64,
    /// Human-readable description of the finding.
    pub description: String,
    /// Supporting evidence items.
    pub evidence: Vec<Evidence>,
    /// The process associated with the detection, if applicable.
    pub affected_process: Option<ProcessInfo>,
    /// When the detection was raised.
    pub timestamp: DateTime<Utc>,
    /// MITRE ATT&CK technique ID (e.g., `"T1059.001"`), if mapped.
    pub mitre_technique: Option<String>,
}

// ---------------------------------------------------------------------------
// Remediation
// ---------------------------------------------------------------------------

/// An action the EDR can take in response to a detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RemediationAction {
    /// Terminate a process by PID.
    KillProcess(u32),
    /// Move a file to quarantine.
    QuarantineFile(String),
    /// Block outbound traffic to a specific host and port.
    BlockNetwork(String, u16),
    /// Remove a registry value (key path, value name).
    RemoveRegistryKey(String, String),
    /// Delete a Windows scheduled task by name.
    RemoveScheduledTask(String),
    /// Stop a Windows service by name.
    StopService(String),
    /// Emit an alert without taking automated action.
    AlertOnly(String),
}

// ---------------------------------------------------------------------------
// Event Priority
// ---------------------------------------------------------------------------

/// Priority assigned to events in the internal processing pipeline.
///
/// Implements [`Ord`] so higher priorities sort after lower ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum EventPriority {
    /// Background / best-effort processing.
    Low,
    /// Standard processing priority.
    Normal,
    /// Elevated — processed before [`Low`](Self::Low) and
    /// [`Normal`](Self::Normal) events.
    High,
    /// Highest priority — processed immediately.
    Critical,
}

// ---------------------------------------------------------------------------
// System Health
// ---------------------------------------------------------------------------

/// Snapshot of EDR system health metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    /// Current CPU usage percentage (0.0 – 100.0).
    pub cpu_usage: f32,
    /// Current resident memory usage in megabytes.
    pub memory_usage_mb: f32,
    /// Telemetry events processed per second (moving average).
    pub events_per_second: f64,
    /// Internal channel fill level as a percentage (0.0 – 100.0).
    pub channel_fill_percent: f32,
    /// Cumulative count of events dropped due to back-pressure.
    pub dropped_events: u64,
    /// Seconds since the EDR service started.
    pub uptime_seconds: u64,
}
