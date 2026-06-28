use std::fmt;

#[derive(Debug, Clone)]
pub enum ScannerError {
    RegistryAccessDenied,
    KeyNotFound,
    NativeError(String),
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RegistryAccessDenied => write!(f, "Registry access denied"),
            Self::KeyNotFound => write!(f, "Registry key not found"),
            Self::NativeError(msg) => write!(f, "Native Windows error: {}", msg),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SuspiciousRegistryEntry {
    pub key_path: String,
    pub value_name: String,
    pub target_path: String,
    pub reason: String,
}

pub struct PersistenceScanner;

impl PersistenceScanner {
    pub fn scan_run_keys() -> Result<Vec<SuspiciousRegistryEntry>, ScannerError> {
        let mut suspicious = Vec::new();

        #[cfg(target_os = "windows")]
        {
            use winreg::enums::*;
            use winreg::RegKey;

            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let run_key_path = r#"Software\Microsoft\Windows\CurrentVersion\Run"#;
            
            if let Ok(run_key) = hkcu.open_subkey_with_flags(run_key_path, KEY_READ) {
                for value_result in run_key.enum_values() {
                    if let Ok((name, value)) = value_result {
                        let target_path = value.to_string();
                        let target_lower = target_path.to_lowercase();
                            
                            // Check if the path points to suspicious locations like AppData or Temp
                            if target_lower.contains("appdata") 
                                || target_lower.contains("temp")
                                || target_lower.contains("downloads") 
                            {
                                suspicious.push(SuspiciousRegistryEntry {
                                    key_path: format!(r#"HKCU\{}"#, run_key_path),
                                    value_name: name,
                                    target_path,
                                    reason: "Suspicious auto-run location (AppData/Temp/Downloads)".to_string(),
                                });
                            }
                    }
                }
            }
        }

        Ok(suspicious)
    }
}
