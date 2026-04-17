use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Config {
    pub values: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.values.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.values.get(key)
    }
}
