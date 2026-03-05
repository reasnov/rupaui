use std::collections::HashMap;

/// A container for dynamic component attributes (e.g., data-* attributes).
/// Separated from Style to maintain strict Separation of Concerns (SOC).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Attributes {
    pub map: HashMap<String, String>,
}

impl Attributes {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets an attribute value.
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.map.insert(key.into(), value.into());
    }

    /// Gets an attribute value.
    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }
}
