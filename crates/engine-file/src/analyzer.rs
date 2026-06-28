use shared_models::Timestamp;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RansomwareSignal {
    pub file_path: String,
    pub timestamp: Timestamp,
    pub pid: u32,
    pub extension: String,
}

pub struct FileAnalyzer;

impl FileAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze_file_io(&self, file_path: &str, pid: u32, timestamp: Timestamp) -> Option<RansomwareSignal> {
        let path = std::path::Path::new(file_path);
        
        if let Some(ext) = path.extension() {
            if let Some(ext_str) = ext.to_str() {
                let ext_lower = ext_str.to_lowercase();
                
                // Common ransomware extensions
                let dangerous_extensions = [
                    "ryuk",
                    "wannacry",
                    "encrypted",
                    "locky",
                    "crypt",
                    "locked",
                ];

                if dangerous_extensions.contains(&ext_lower.as_str()) {
                    return Some(RansomwareSignal {
                        file_path: file_path.to_string(),
                        timestamp,
                        pid,
                        extension: ext_lower,
                    });
                }
            }
        }
        None
    }
}
