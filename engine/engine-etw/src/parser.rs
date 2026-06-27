/// The raw, parsed ETW event directly emitted from the C-callback.
/// This structure isolates C-pointers from the rest of the application.
#[derive(Debug)]
pub struct RawEtwEvent {
    pub provider_id: windows::core::GUID,
    pub event_id: u16,
    pub opcode: u8,
    pub process_id: u32,
    pub thread_id: u32,
    pub timestamp: i64,
    // Note: We avoid heavy heap allocations here. 
    // In a full implementation, we'd extract specific fields (like String paths) 
    // dynamically using TDH and store them here as owned `String`s.
    // For this structural phase, we represent the payload generically.
    pub raw_payload_extracted: bool, 
}

pub mod parse {
    use super::RawEtwEvent;
    use crate::metrics;
    use windows::Win32::System::Diagnostics::Etw::EVENT_RECORD;

    /// Converts the raw C struct `EVENT_RECORD` into an owned `RawEtwEvent`.
    /// 
    /// **Hot Path Constraint:** Must perform exactly zero heap allocations unless 
    /// cloning extracted strings (e.g., file paths) from the event buffer.
    pub fn parse_event(record: &EVENT_RECORD) -> Option<RawEtwEvent> {
        metrics::inc_received();

        // Safety: We are accessing fields of the EVENT_RECORD provided by the OS.
        let header = &record.EventHeader;
        
        // Simulating parsing (without actual TDH property extraction for now).
        let raw = RawEtwEvent {
            provider_id: header.ProviderId,
            event_id: header.EventDescriptor.Id,
            opcode: header.EventDescriptor.Opcode,
            process_id: header.ProcessId,
            thread_id: header.ThreadId,
            timestamp: header.TimeStamp,
            raw_payload_extracted: true,
        };

        metrics::inc_parsed();
        Some(raw)
    }
}
