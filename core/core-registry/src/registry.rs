use std::collections::HashMap;
use crate::models::{CapabilityId, Capability};

pub struct CapabilityRegistry {
    capabilities: HashMap<String, Box<dyn Capability>>,
}

impl CapabilityRegistry {
    pub fn new() -> Self {
        Self {
            capabilities: HashMap::new(),
        }
    }

    pub fn register(&mut self, capability: Box<dyn Capability>) {
        let meta = capability.metadata();
        self.capabilities.insert(meta.id.0.clone(), capability);
    }

    pub fn get_capability(&self, id: &CapabilityId) -> Option<&Box<dyn Capability>> {
        self.capabilities.get(&id.0)
    }

    pub fn get_all_metadata(&self) -> Vec<crate::models::CapabilityMetadata> {
        self.capabilities.values().map(|c| c.metadata()).collect()
    }

    pub fn validate_dependencies(&self) -> Result<(), String> {
        for (id, cap) in &self.capabilities {
            for dep in &cap.metadata().dependencies {
                if !self.capabilities.contains_key(&dep.0) {
                    return Err(format!("Capability {} depends on missing capability {}", id, dep.0));
                }
            }
        }
        Ok(())
    }
}
