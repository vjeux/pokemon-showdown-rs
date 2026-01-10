//! ID type - the core identifier used throughout Pokemon Showdown

use serde::{Deserialize, Serialize};

use super::to_id;

/// An ID must be lowercase alphanumeric.
/// This is the core identifier type used throughout Pokemon Showdown.
/// JavaScript equivalent: ID (sim/global-types.ts)
/// JavaScript uses string type with lowercase alphanumeric normalization
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct ID(String);

impl ID {
    /// Create a new ID from a string, converting to lowercase and removing non-alphanumeric chars
    pub fn new(text: &str) -> Self {
        ID(to_id(text))
    }

    /// Create an empty ID
    pub fn empty() -> Self {
        ID(String::new())
    }

    /// Check if the ID is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get the inner string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for ID {
    fn from(s: &str) -> Self {
        ID::new(s)
    }
}

impl From<String> for ID {
    fn from(s: String) -> Self {
        ID::new(&s)
    }
}

impl std::fmt::Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
