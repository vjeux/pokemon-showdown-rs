//! OHKO type for one-hit knockout moves

use serde::Serialize;

/// OHKO can be true (generic OHKO) or a string (type-based OHKO like "Ice")
/// TODO: Rust uses enum to represent JavaScript's true | string union type
/// JavaScript: ohko?: true | string
#[derive(Debug, Clone)]
pub enum Ohko {
    Generic,           // true
    TypeBased(String), // Type name like "Ice", "Normal"
}

impl Serialize for Ohko {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Ohko::Generic => serializer.serialize_bool(true),
            Ohko::TypeBased(s) => serializer.serialize_str(s),
        }
    }
}
