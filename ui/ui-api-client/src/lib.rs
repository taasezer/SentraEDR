use core_eventbus::event_bus::EventBus;
use core_eventbus::models::EventMessage;
use std::sync::Arc;

pub struct ApiClient<T: EventMessage + 'static> {
    // Decouples UI from concrete runtime implementations. Allows injecting a MockEventBus
    event_bus: Arc<EventBus<T>>,
}

impl<T: EventMessage + 'static> ApiClient<T> {
    pub fn new(event_bus: Arc<EventBus<T>>) -> Self {
        Self { event_bus }
    }

    pub fn subscribe_alerts(&self) {
        // subscribes to the EventBus and maps internal normalized telemetry into AlertView
    }
}
