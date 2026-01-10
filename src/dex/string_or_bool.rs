//! StringOrBool type for union type fields

use serde::{Deserialize, Serialize};

/// StringOrBool can be a boolean or a string
/// Rust uses enum to represent JavaScript's boolean | string union type
/// JavaScript: field: boolean | string
#[derive(Debug, Clone, Default)]
pub enum StringOrBool {
    #[default]
    None,
    Bool(bool),
    String(String),
}

impl<'de> Deserialize<'de> for StringOrBool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, Visitor};

        struct StringOrBoolVisitor;

        impl<'de> Visitor<'de> for StringOrBoolVisitor {
            type Value = StringOrBool;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a boolean or string")
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StringOrBool::Bool(value))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StringOrBool::String(value.to_string()))
            }
        }

        deserializer.deserialize_any(StringOrBoolVisitor)
    }
}

impl Serialize for StringOrBool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            StringOrBool::None => serializer.serialize_none(),
            StringOrBool::Bool(b) => serializer.serialize_bool(*b),
            StringOrBool::String(s) => serializer.serialize_str(s),
        }
    }
}

impl StringOrBool {
    /// Returns true if this is a boolean value set to true
    pub fn is_true(&self) -> bool {
        matches!(self, StringOrBool::Bool(true))
    }

    /// Returns true if this is Some value (either bool or string)
    pub fn is_some(&self) -> bool {
        !matches!(self, StringOrBool::None)
    }
}
