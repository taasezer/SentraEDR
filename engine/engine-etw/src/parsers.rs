pub trait EventRecordParser {
    // Must implement Zero-Copy parsing utilizing &[u8] spans directly from EVENT_RECORD
}

pub struct ProcessParser;
pub struct ImageParser;
pub struct RegistryParser;
