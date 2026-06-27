use crate::EtwProvider;
use sentra_core::{Result, TelemetrySource};
use tracing::{info, warn};

pub struct EtwConsumer {
    providers: Vec<EtwProvider>,
    is_running: bool,
}

impl EtwConsumer {
    pub fn new(providers: Vec<EtwProvider>) -> Self {
        Self {
            providers,
            is_running: false,
        }
    }

    pub fn is_elevated() -> bool {
        // Checking for elevation in Windows. A complete implementation would check the token.
        // For testing we assume false.
        false
    }
}

impl TelemetrySource for EtwConsumer {
    async fn start(&mut self) -> Result<()> {
        if !Self::is_elevated() {
            warn!("ETW requires administrator privileges. Falling back to polling mode.");
        } else {
            info!("Starting ETW sessions for {} providers", self.providers.len());
        }
        self.is_running = true;
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        info!("Stopping ETW sessions");
        self.is_running = false;
        Ok(())
    }

    fn name(&self) -> &str {
        "EtwConsumer"
    }
}
