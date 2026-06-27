use std::collections::HashMap;
use crate::providers::ActionProvider;

/// Pluggable registry mapping `provider_id`s to actual providers.
pub struct ActionRegistry {
    providers: HashMap<String, Box<dyn ActionProvider>>,
}

impl ActionRegistry {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    pub fn register(&mut self, provider: Box<dyn ActionProvider>) {
        self.providers.insert(provider.provider_id().to_string(), provider);
    }

    pub fn get(&self, id: &str) -> Option<&Box<dyn ActionProvider>> {
        self.providers.get(id)
    }
}
