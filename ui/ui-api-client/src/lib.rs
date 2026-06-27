use core_eventbus::event_bus::EventBus;
use std::sync::Arc;
use ui_models::AlertView;

pub struct ApiClient {
    // Decouples UI from concrete runtime implementations. Allows injecting a MockEventBus
    event_bus: Arc<dyn EventBus>,
}

impl ApiClient {
    pub fn new(event_bus: Arc<dyn EventBus>) -> Self {
        Self { event_bus }
    }

    pub fn subscribe_alerts(&self) {
        // subscribes to the EventBus and maps internal normalized telemetry into AlertView
    }
}
