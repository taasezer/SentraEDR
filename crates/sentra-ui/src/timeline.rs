use shared_models::Timestamp;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimelineKind {
    AlertObserved,
    ActionQueued,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimelineEntry {
    pub timestamp: Timestamp,
    pub kind: TimelineKind,
    pub title: String,
}

impl TimelineEntry {
    pub fn new(timestamp: Timestamp, kind: TimelineKind, title: impl Into<String>) -> Self {
        Self {
            timestamp,
            kind,
            title: title.into(),
        }
    }
}
