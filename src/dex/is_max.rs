//! IsMax type for Max Move identification

use serde::Serialize;

/// IsMax can be true (generic Max move) or a string (species-specific G-Max move)
/// TODO: Rust uses enum to represent JavaScript's true | string union type
/// JavaScript: isMax?: true | string
#[derive(Debug, Clone)]
pub enum IsMax {
    Generic,         // true
    Species(String), // Pokemon name like "Butterfree"
}

impl Serialize for IsMax {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            IsMax::Generic => serializer.serialize_bool(true),
            IsMax::Species(s) => serializer.serialize_str(s),
        }
    }
}
