use crate::make_detection;
use sentra_core::{DetectionResult, Result, SentraError, ThreatLevel};
use std::path::Path;

pub struct StartupEntry {
    pub file_name: String,
    pub full_path: String,
    pub target: String,
    pub is_shortcut: bool,
}

pub fn get_startup_folders() -> Result<Vec<String>> {
    let mut folders = Vec::new();
    
    // Using environment variables for simplicity in this implementation
    if let Ok(appdata) = std::env::var("APPDATA") {
        let path = format!("{}\\Microsoft\\Windows\\Start Menu\\Programs\\Startup", appdata);
        folders.push(path);
    }
    
    if let Ok(programdata) = std::env::var("PROGRAMDATA") {
        let path = format!("{}\\Microsoft\\Windows\\Start Menu\\Programs\\Startup", programdata);
        folders.push(path);
    }
    
    Ok(folders)
}

pub fn scan_startup_folder(path: &str) -> Result<Vec<StartupEntry>> {
    let mut entries = Vec::new();
    let dir = Path::new(path);
    
    if dir.exists() && dir.is_dir() {
        if let Ok(read_dir) = std::fs::read_dir(dir) {
            for entry in read_dir.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        let file_name = entry.file_name().to_string_lossy().to_string();
                        let full_path = entry.path().to_string_lossy().to_string();
                        let is_shortcut = file_name.ends_with(".lnk");
                        
                        entries.push(StartupEntry {
                            file_name,
                            full_path,
                            target: String::new(), // Extracting target from .lnk requires COM IShellLink
                            is_shortcut,
                        });
                    }
                }
            }
        }
    }
    
    Ok(entries)
}

pub fn detect_suspicious_startup(entries: &[StartupEntry]) -> Vec<DetectionResult> {
    let mut detections = Vec::new();

    for entry in entries {
        let name = entry.file_name.to_lowercase();
        
        if name.ends_with(".vbs") || name.ends_with(".js") || name.ends_with(".bat") || name.ends_with(".cmd") || name.ends_with(".ps1") {
            detections.push(make_detection(
                "Suspicious Startup File",
                &format!("Script file in startup folder: {}", entry.file_name),
                ThreatLevel::High,
                0.7,
                &format!("Path: {}", entry.full_path),
                Some("T1547.001"),
            ));
        }
    }

    detections
}
