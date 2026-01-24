//! ID type - the core identifier used throughout Pokemon Showdown

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::to_id;

/// An ID must be lowercase alphanumeric.
/// This is the core identifier type used throughout Pokemon Showdown.
/// JavaScript equivalent: ID (sim/global-types.ts)
/// JavaScript uses string type with lowercase alphanumeric normalization
///
/// Uses Arc<str> internally for cheap cloning (reference count increment instead of heap allocation).
/// Arc is used instead of Rc for thread-safety (required for static Lazy initialization).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ID(Arc<str>);

impl ID {
    /// Create a new ID from a string, converting to lowercase and removing non-alphanumeric chars
    pub fn new(text: &str) -> Self {
        let normalized = to_id(text);
        ID(Arc::from(normalized))
    }

    /// Create an empty ID
    pub fn empty() -> Self {
        ID(Arc::from(""))
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

impl Default for ID {
    fn default() -> Self {
        ID::empty()
    }
}

// Custom serialization - serialize as plain string
impl Serialize for ID {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

// Custom deserialization - deserialize from plain string
impl<'de> Deserialize<'de> for ID {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(ID(Arc::from(s)))
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
