/// Definitions of ETW Providers.

use windows::core::GUID;

/// Represents an ETW Provider subscription configuration.
pub struct ProviderConfig {
    pub guid: GUID,
    pub level: u8,
    pub match_any_keyword: u64,
    pub match_all_keyword: u64,
}

pub const KERNEL_PROCESS_PROVIDER: GUID = GUID::from_u128(0x22fb2cd6_0e7b_422b_a0c7_2fad1fd0e716);
pub const POWERSHELL_PROVIDER: GUID = GUID::from_u128(0xa0c1853b_5c40_4b15_8766_3cf1c58f985a);

impl ProviderConfig {
    pub fn new(guid: GUID) -> Self {
        Self {
            guid,
            level: 5, // TRACE_LEVEL_VERBOSE
            match_any_keyword: 0xFFFFFFFFFFFFFFFF,
            match_all_keyword: 0,
        }
    }
}
