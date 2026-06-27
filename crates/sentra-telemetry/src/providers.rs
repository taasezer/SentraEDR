use chrono::{DateTime, Utc};

pub const MICROSOFT_WINDOWS_KERNEL_PROCESS: &str = "22fb2cd6-0e7b-422b-a0c7-2fad1fd0e716";
pub const MICROSOFT_WINDOWS_KERNEL_REGISTRY: &str = "70eb4f03-c1de-4f73-a051-33d13d5413bd";
pub const MICROSOFT_WINDOWS_KERNEL_FILE: &str = "edd08927-9cc4-4e65-b970-c2560fb5c289";
pub const MICROSOFT_WINDOWS_KERNEL_NETWORK: &str = "7dd42a49-5329-4832-8dfd-43d979153a88";
pub const MICROSOFT_WINDOWS_POWERSHELL: &str = "a0c1853b-5c40-4b15-8766-3cf1c58f985a";
pub const MICROSOFT_WINDOWS_DNS_CLIENT: &str = "1c95126e-7eea-49a9-a3fe-a378b03ddb4d";

#[derive(Debug, Clone)]
pub struct EtwProvider {
    pub guid: String,
    pub name: String,
    pub keywords: u64,
    pub level: u8,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct EtwProviderStatus {
    pub provider: EtwProvider,
    pub is_active: bool,
    pub events_received: u64,
    pub events_lost: u64,
    pub last_event_time: Option<DateTime<Utc>>,
}

pub fn get_default_providers() -> Vec<EtwProvider> {
    vec![
        EtwProvider {
            guid: MICROSOFT_WINDOWS_KERNEL_PROCESS.to_string(),
            name: "Kernel Process".to_string(),
            keywords: 0x10, // Example
            level: 4,
            description: "Process and thread creation".to_string(),
        },
        EtwProvider {
            guid: MICROSOFT_WINDOWS_KERNEL_REGISTRY.to_string(),
            name: "Kernel Registry".to_string(),
            keywords: 0xFFFF,
            level: 4,
            description: "Registry modifications".to_string(),
        },
        EtwProvider {
            guid: MICROSOFT_WINDOWS_POWERSHELL.to_string(),
            name: "PowerShell".to_string(),
            keywords: 0xFFFF,
            level: 4,
            description: "PowerShell script blocks".to_string(),
        },
    ]
}
