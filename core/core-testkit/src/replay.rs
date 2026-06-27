pub enum ReplayMode {
    RealTime,
    Accelerated,
    StepByStep,
    Deterministic,
    Stress,
}

pub struct TelemetryReplayHarness {
    pub mode: ReplayMode,
}

impl TelemetryReplayHarness {
    pub fn new(mode: ReplayMode) -> Self {
        Self { mode }
    }
    
    pub fn replay_golden_dataset(&self) {
        // Pushes JSON telemetry onto the mock EventBus using the abstract Clock
    }
}
