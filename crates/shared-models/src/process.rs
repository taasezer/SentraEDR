use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImagePath(String);

impl ImagePath {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandLine(String);

impl CommandLine {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessIdentity {
    pub process_id: u32,
    pub parent_process_id: Option<u32>,
    pub image_path: Option<ImagePath>,
    pub command_line: Option<CommandLine>,
    pub user_sid: Option<String>,
}

impl ProcessIdentity {
    pub fn new(process_id: u32) -> Self {
        Self {
            process_id,
            parent_process_id: None,
            image_path: None,
            command_line: None,
            user_sid: None,
        }
    }

    pub fn with_parent(mut self, parent_process_id: u32) -> Self {
        self.parent_process_id = Some(parent_process_id);
        self
    }

    pub fn with_image_path(mut self, image_path: ImagePath) -> Self {
        self.image_path = Some(image_path);
        self
    }

    pub fn with_command_line(mut self, command_line: CommandLine) -> Self {
        self.command_line = Some(command_line);
        self
    }
}
