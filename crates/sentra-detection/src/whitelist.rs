use chrono::{DateTime, Utc};
use sentra_core::TelemetryEvent;
use std::collections::{HashMap, HashSet};

pub struct Whitelist {
    pub whitelisted_paths: HashSet<String>,
    pub whitelisted_process_names: HashSet<String>,
    pub whitelisted_hashes: HashSet<String>,
    pub suppressed_rules: HashMap<String, DateTime<Utc>>,
}

impl Whitelist {
    pub fn new() -> Self {
        Self {
            whitelisted_paths: HashSet::new(),
            whitelisted_process_names: HashSet::new(),
            whitelisted_hashes: HashSet::new(),
            suppressed_rules: HashMap::new(),
        }
    }

    pub fn is_whitelisted(&self, _event: &TelemetryEvent) -> bool {
        // Implement whitelisting checks
        false
    }

    pub fn add_path(&mut self, path: String) {
        self.whitelisted_paths.insert(path.to_lowercase());
    }

    pub fn suppress_rule(&mut self, rule_name: String, until: DateTime<Utc>) {
        self.suppressed_rules.insert(rule_name, until);
    }
}
