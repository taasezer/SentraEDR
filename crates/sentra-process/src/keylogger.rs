use sentra_core::{DetectionResult, Result};

pub struct KeyloggerDetector {}

impl KeyloggerDetector {
    pub fn new() -> Self {
        Self {}
    }

    pub fn detect_keyboard_hooks() -> Result<Vec<DetectionResult>> {
        Ok(Vec::new())
    }

    pub fn detect_rapid_keystate_polling(&self, _pid: u32) -> bool {
        false
    }

    pub fn detect_raw_input_registration() -> Result<Vec<u32>> {
        Ok(Vec::new())
    }
}
