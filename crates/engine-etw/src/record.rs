use shared_models::{CommandLine, ImagePath, Timestamp};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EtwProcessEventKind {
    Start,
    Exit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EtwProcessRecord {
    pub event_kind: EtwProcessEventKind,
    pub timestamp: Timestamp,
    pub process_id: u32,
    pub parent_process_id: Option<u32>,
    pub image_path: Option<ImagePath>,
    pub command_line: Option<CommandLine>,
    pub confidence: u8,
}

impl EtwProcessRecord {
    pub fn new(event_kind: EtwProcessEventKind, timestamp: Timestamp, process_id: u32) -> Self {
        Self {
            event_kind,
            timestamp,
            process_id,
            parent_process_id: None,
            image_path: None,
            command_line: None,
            confidence: 100,
        }
    }

    pub fn with_parent_process_id(mut self, parent_process_id: u32) -> Self {
        self.parent_process_id = Some(parent_process_id);
        self
    }

    pub fn with_image_path(mut self, image_path: impl Into<String>) -> Self {
        self.image_path = Some(ImagePath::new(image_path));
        self
    }

    pub fn with_command_line(mut self, command_line: impl Into<String>) -> Self {
        self.command_line = Some(CommandLine::new(command_line));
        self
    }

    pub fn with_confidence(mut self, confidence: u8) -> Self {
        self.confidence = confidence;
        self
    }
}
