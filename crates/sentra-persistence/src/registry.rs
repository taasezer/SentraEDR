use crate::make_detection;
use sentra_core::{DetectionResult, Result, SentraError, ThreatLevel};
use windows_registry::CURRENT_USER;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PersistenceType {
    RunKey,
    RunOnce,
    WinlogonShell,
    WinlogonUserinit,
    ShellFolder,
    EnvironmentScript,
    ImageFileExecution,
}

pub struct RegistryPersistenceEntry {
    pub hive: String,
    pub key_path: String,
    pub value_name: String,
    pub value_data: String,
    pub entry_type: PersistenceType,
}

pub const PERSISTENCE_KEYS: &[(&str, &str, PersistenceType)] = &[
    ("HKCU", "Software\\Microsoft\\Windows\\CurrentVersion\\Run", PersistenceType::RunKey),
    ("HKCU", "Software\\Microsoft\\Windows\\CurrentVersion\\RunOnce", PersistenceType::RunOnce),
    ("HKLM", "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run", PersistenceType::RunKey),
    ("HKLM", "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\RunOnce", PersistenceType::RunOnce),
    ("HKLM", "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Winlogon", PersistenceType::WinlogonShell),
    ("HKCU", "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\User Shell Folders", PersistenceType::ShellFolder),
    ("HKLM", "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Shell Folders", PersistenceType::ShellFolder),
    ("HKCU", "Environment", PersistenceType::EnvironmentScript),
];

pub fn scan_registry_persistence() -> Result<Vec<RegistryPersistenceEntry>> {
    let mut entries = Vec::new();

    for (hive_name, path, p_type) in PERSISTENCE_KEYS {
        // Simplified approach: only check HKCU for this implementation since HKLM requires elevation
        if *hive_name == "HKCU" {
            if let Ok(key) = CURRENT_USER.open(path) {
                if let Ok(keys) = key.values() {
                    for (name, value) in keys {
                        if let Ok(val_str) = value.try_into() {
                            entries.push(RegistryPersistenceEntry {
                                hive: hive_name.to_string(),
                                key_path: path.to_string(),
                                value_name: name.to_string(),
                                value_data: val_str,
                                entry_type: p_type.clone(),
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(entries)
}

pub fn detect_suspicious_entries(entries: &[RegistryPersistenceEntry]) -> Vec<DetectionResult> {
    let mut detections = Vec::new();

    for entry in entries {
        let data = entry.value_data.to_lowercase();
        
        if data.contains("\\temp\\") || data.contains("\\appdata\\local\\temp\\") {
            detections.push(make_detection(
                "Suspicious Registry Persistence",
                &format!("Persistence entry points to temp directory: {}", entry.value_name),
                ThreatLevel::High,
                0.8,
                &format!("Path: {}\\{}\\{} -> {}", entry.hive, entry.key_path, entry.value_name, entry.value_data),
                Some("T1547.001"),
            ));
        }

        if data.contains("-enc ") || data.contains("-encodedcommand") {
            detections.push(make_detection(
                "Encoded PowerShell Persistence",
                &format!("Persistence entry uses encoded PowerShell: {}", entry.value_name),
                ThreatLevel::Critical,
                0.9,
                &format!("Path: {}\\{}\\{} -> {}", entry.hive, entry.key_path, entry.value_name, entry.value_data),
                Some("T1547.001"),
            ));
        }

        if data.ends_with(".js") || data.ends_with(".vbs") || data.ends_with(".hta") {
            detections.push(make_detection(
                "Script-based Registry Persistence",
                &format!("Persistence entry points to script file: {}", entry.value_name),
                ThreatLevel::High,
                0.7,
                &format!("Path: {}\\{}\\{} -> {}", entry.hive, entry.key_path, entry.value_name, entry.value_data),
                Some("T1547.001"),
            ));
        }
    }

    detections
}
