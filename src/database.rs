use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pairs: HashMap<String, String>,
}

#[allow(unused)]
impl Database {
    pub fn new() -> Self {
        Self { pairs: HashMap::new() }
    }

    pub fn has_name(&self, key: &str) -> bool {
        self.pairs.contains_key(key)
    }

    pub fn count(&self) -> usize {
        self.pairs.len()
    }

    pub fn list_all(&self) -> Vec<String> {
        self.pairs.iter().map(|v| v.0.to_string()).collect()
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        self.pairs.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.pairs.get(key)
    }

    pub fn get_unchecked(&self, key: &str) -> &String {
        self.get(key).unwrap()
    }

    pub fn update(&mut self, key: &str, value: &str) {
        self.pairs.insert(key.to_string(), value.to_string());
    }

    pub fn remove(&mut self, key: &str) {
        self.pairs.remove(key);
    }
}
