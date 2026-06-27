use crate::event_metadata::EventMetadata;

pub trait EventSink: Send + Sync {
    fn emit(&self, metadata: EventMetadata, message: &str);
}

pub struct MockEventSink;

impl EventSink for MockEventSink {
    fn emit(&self, metadata: EventMetadata, message: &str) {
        // Dev/Testing sink. Future implementation: WindowsEventLogSink
    }
}
