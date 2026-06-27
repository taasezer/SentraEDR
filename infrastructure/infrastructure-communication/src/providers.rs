use core_eventbus::models::{CommandMessage, EventMessage};
use core_registry::models::Capability;

pub trait CommunicationProvider: Capability {
    fn provider_id(&self) -> &str;
    fn connection_type(&self) -> &str; // e.g. "Local", "NamedPipe", "gRPC"

    // Providers could eventually expose generic routing logic here,
    // but the actual `EventBus` and `CommandBus` structs manage the bounded queues locally.
}

pub struct LocalCommunicationProvider {
    // In-process transport implementation wrapper
}

// In a real implementation, LocalCommunicationProvider implements Capability and CommunicationProvider.
