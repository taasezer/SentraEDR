//! Configuration subsystem for SentraEDR.
//!
//! [`SentraConfig`] aggregates all component-level configs and can be
//! loaded from a TOML file via [`SentraConfig::load`].  Sensible
//! production defaults are provided by [`Default`] impls so the EDR
//! can start even without a config file.

use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::error::{Result, SentraError};
use crate::types::ThreatLevel;

// ---------------------------------------------------------------------------
// Top-level config
// ---------------------------------------------------------------------------

/// Root configuration for the SentraEDR platform.
///
/// Each field corresponds to a logical subsystem. All sub-configs
/// carry their own [`Default`] implementation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentraConfig {
    /// General / global settings.
    #[serde(default)]
    pub general: GeneralConfig,
    /// Detection engine settings.
    #[serde(default)]
    pub detection: DetectionConfig,
    /// Telemetry pipeline settings.
    #[serde(default)]
    pub telemetry: TelemetryConfig,
    /// Process monitoring settings.
    #[serde(default)]
    pub process: ProcessConfig,
    /// Network monitoring settings.
    #[serde(default)]
    pub network: NetworkConfig,
    /// Persistence monitoring settings.
    #[serde(default)]
    pub persistence: PersistenceConfig,
    /// Remediation settings.
    #[serde(default)]
    pub remediation: RemediationConfig,
}

impl SentraConfig {
    /// Load configuration from a TOML file at `path`.
    ///
    /// Returns [`SentraError::Configuration`] if the file cannot be
    /// read or the TOML content is invalid.
    pub fn load(path: &str) -> Result<Self> {
        let path = Path::new(path);
        let content = std::fs::read_to_string(path).map_err(|e| {
            SentraError::Configuration(format!(
                "failed to read config file '{}': {e}",
                path.display()
            ))
        })?;

        let config: Self = toml_edit::de::from_str(&content).map_err(|e| {
            SentraError::Configuration(format!(
                "failed to parse config file '{}': {e}",
                path.display()
            ))
        })?;

        tracing::info!(path = %path.display(), "loaded configuration");
        Ok(config)
    }
}

impl Default for SentraConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            detection: DetectionConfig::default(),
            telemetry: TelemetryConfig::default(),
            process: ProcessConfig::default(),
            network: NetworkConfig::default(),
            persistence: PersistenceConfig::default(),
            remediation: RemediationConfig::default(),
        }
    }
}

// ---------------------------------------------------------------------------
// Sub-configs
// ---------------------------------------------------------------------------

/// General / global settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Tracing / log level filter string (e.g., `"info"`, `"debug"`).
    pub log_level: String,
    /// Base directory for data files (logs, quarantine, state).
    pub data_dir: String,
    /// Maximum resident memory (MB) before back-pressure is applied.
    pub max_memory_mb: u64,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            log_level: "info".to_owned(),
            data_dir: r"C:\ProgramData\SentraEDR".to_owned(),
            max_memory_mb: 512,
        }
    }
}

/// Detection engine configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionConfig {
    /// List of rule names (or glob patterns) to enable.
    pub enabled_rules: Vec<String>,
    /// Minimum threat level that triggers an alert.
    pub threat_threshold: ThreatLevel,
    /// Sliding correlation window in seconds.
    pub correlation_window_secs: u64,
    /// Maximum events buffered in the correlation window.
    pub max_events_in_window: usize,
}

impl Default for DetectionConfig {
    fn default() -> Self {
        Self {
            enabled_rules: vec!["*".to_owned()],
            threat_threshold: ThreatLevel::Medium,
            correlation_window_secs: 60,
            max_events_in_window: 10_000,
        }
    }
}

/// Telemetry pipeline settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryConfig {
    /// Whether ETW-based telemetry is enabled.
    pub etw_enabled: bool,
    /// Base polling interval for non-ETW sources (milliseconds).
    pub polling_interval_ms: u64,
    /// Capacity of the internal bounded event channel.
    pub channel_capacity: usize,
    /// Maximum sustained event rate before back-pressure.
    pub max_events_per_second: u64,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            etw_enabled: true,
            polling_interval_ms: 1_000,
            channel_capacity: 50_000,
            max_events_per_second: 10_000,
        }
    }
}

/// Process monitoring configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessConfig {
    /// Whether process monitoring is enabled.
    pub monitor_enabled: bool,
    /// Process scan / poll interval in milliseconds.
    pub scan_interval_ms: u64,
    /// Whether to track parent→child relationships.
    pub track_children: bool,
    /// Process names considered suspicious as parents (lowercase).
    pub suspicious_parents: Vec<String>,
}

impl Default for ProcessConfig {
    fn default() -> Self {
        Self {
            monitor_enabled: true,
            scan_interval_ms: 2_000,
            track_children: true,
            suspicious_parents: vec![
                "cmd.exe".to_owned(),
                "powershell.exe".to_owned(),
                "pwsh.exe".to_owned(),
                "wscript.exe".to_owned(),
                "cscript.exe".to_owned(),
                "mshta.exe".to_owned(),
                "regsvr32.exe".to_owned(),
                "rundll32.exe".to_owned(),
            ],
        }
    }
}

/// Network monitoring configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Whether network monitoring is enabled.
    pub monitor_enabled: bool,
    /// Network table scan interval in milliseconds.
    pub scan_interval_ms: u64,
    /// Ports considered suspicious for outbound connections.
    pub suspicious_ports: Vec<u16>,
    /// Known C2 indicators (domains or IP addresses).
    pub c2_indicators: Vec<String>,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            monitor_enabled: true,
            scan_interval_ms: 5_000,
            suspicious_ports: vec![4444, 5555, 8443, 1337, 9001, 6666, 31337],
            c2_indicators: Vec::new(),
        }
    }
}

/// Persistence mechanism monitoring configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceConfig {
    /// Whether persistence monitoring is enabled.
    pub monitor_enabled: bool,
    /// Scan interval in milliseconds.
    pub scan_interval_ms: u64,
    /// Monitor registry Run / RunOnce keys.
    pub registry_watch: bool,
    /// Monitor Windows services.
    pub service_watch: bool,
    /// Monitor scheduled tasks.
    pub task_watch: bool,
    /// Monitor startup folder entries.
    pub startup_watch: bool,
}

impl Default for PersistenceConfig {
    fn default() -> Self {
        Self {
            monitor_enabled: true,
            scan_interval_ms: 30_000,
            registry_watch: true,
            service_watch: true,
            task_watch: true,
            startup_watch: true,
        }
    }
}

/// Remediation subsystem configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationConfig {
    /// Whether automated remediation is enabled.
    pub auto_remediate: bool,
    /// Threat levels **above** this value require manual confirmation
    /// before auto-remediation proceeds.
    pub require_confirmation_above: ThreatLevel,
    /// If `true`, remediation actions are logged but not executed.
    pub dry_run: bool,
    /// If `true`, every remediation action (including no-ops) is logged.
    pub log_all_actions: bool,
}

impl Default for RemediationConfig {
    fn default() -> Self {
        Self {
            auto_remediate: false,
            require_confirmation_above: ThreatLevel::High,
            dry_run: true,
            log_all_actions: true,
        }
    }
}
