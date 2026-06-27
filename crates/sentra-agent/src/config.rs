use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentConfig {
    pub mode: AgentMode,
    pub queue: QueueConfig,
    #[serde(default)]
    pub ipc: IpcConfig,
    pub logging: LoggingConfig,
}

impl AgentConfig {
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        let contents = fs::read_to_string(path).map_err(|source| ConfigError::ReadFailed {
            path: path.display().to_string(),
            source,
        })?;
        toml::from_str(&contents).map_err(|source| ConfigError::ParseFailed {
            path: path.display().to_string(),
            source,
        })
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.queue.telemetry_capacity == 0 {
            return Err(ConfigError::InvalidValue {
                field: "queue.telemetry_capacity",
                reason: "capacity must be greater than zero",
            });
        }

        if self.queue.detection_capacity == 0 {
            return Err(ConfigError::InvalidValue {
                field: "queue.detection_capacity",
                reason: "capacity must be greater than zero",
            });
        }

        if self.ipc.dispatcher_capacity == 0 {
            return Err(ConfigError::InvalidValue {
                field: "ipc.dispatcher_capacity",
                reason: "capacity must be greater than zero",
            });
        }

        Ok(())
    }
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            mode: AgentMode::ObserveOnly,
            queue: QueueConfig::default(),
            ipc: IpcConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AgentMode {
    ObserveOnly,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueConfig {
    pub telemetry_capacity: usize,
    pub detection_capacity: usize,
}

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            telemetry_capacity: 4096,
            detection_capacity: 1024,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IpcConfig {
    pub enabled: bool,
    pub dispatcher_capacity: usize,
}

impl Default for IpcConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            dispatcher_capacity: 256,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
        }
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to read config file '{path}'")]
    ReadFailed {
        path: String,
        source: std::io::Error,
    },

    #[error("failed to parse config file '{path}'")]
    ParseFailed {
        path: String,
        source: toml::de::Error,
    },

    #[error("invalid config value for {field}: {reason}")]
    InvalidValue {
        field: &'static str,
        reason: &'static str,
    },
}
