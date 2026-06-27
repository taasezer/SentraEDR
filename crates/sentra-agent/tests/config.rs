use sentra_agent::config::{AgentConfig, AgentMode, ConfigError};
use std::fs;

#[test]
fn default_config_is_observe_only() {
    let config = AgentConfig::default();

    assert_eq!(config.mode, AgentMode::ObserveOnly);
    assert_eq!(config.queue.telemetry_capacity, 4096);
    assert_eq!(config.queue.detection_capacity, 1024);
    assert!(config.ipc.enabled);
    assert_eq!(config.ipc.dispatcher_capacity, 256);
    assert!(config.validate().is_ok());
}

#[test]
fn config_loads_from_toml_file() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("sentra-agent.toml");
    fs::write(
        &path,
        r#"
mode = "observe-only"

[queue]
telemetry_capacity = 128
detection_capacity = 64

[ipc]
enabled = true
dispatcher_capacity = 32

[logging]
level = "debug"
"#,
    )
    .unwrap();

    let config = AgentConfig::load_from_file(&path).unwrap();

    assert_eq!(config.mode, AgentMode::ObserveOnly);
    assert_eq!(config.queue.telemetry_capacity, 128);
    assert_eq!(config.queue.detection_capacity, 64);
    assert!(config.ipc.enabled);
    assert_eq!(config.ipc.dispatcher_capacity, 32);
    assert_eq!(config.logging.level, "debug");
}

#[test]
fn config_loads_default_ipc_settings_when_omitted() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("sentra-agent.toml");
    fs::write(
        &path,
        r#"
mode = "observe-only"

[queue]
telemetry_capacity = 128
detection_capacity = 64

[logging]
level = "debug"
"#,
    )
    .unwrap();

    let config = AgentConfig::load_from_file(&path).unwrap();

    assert!(config.ipc.enabled);
    assert_eq!(config.ipc.dispatcher_capacity, 256);
}

#[test]
fn zero_capacity_is_rejected() {
    let mut config = AgentConfig::default();
    config.queue.telemetry_capacity = 0;

    let error = config.validate().unwrap_err();

    assert!(matches!(
        error,
        ConfigError::InvalidValue {
            field: "queue.telemetry_capacity",
            reason: "capacity must be greater than zero"
        }
    ));
}

#[test]
fn zero_ipc_dispatcher_capacity_is_rejected() {
    let mut config = AgentConfig::default();
    config.ipc.dispatcher_capacity = 0;

    let error = config.validate().unwrap_err();

    assert!(matches!(
        error,
        ConfigError::InvalidValue {
            field: "ipc.dispatcher_capacity",
            reason: "capacity must be greater than zero"
        }
    ));
}
