//! StringOrVec type for union type fields

use serde::{Deserialize, Serialize};

/// StringOrVec can be a single string or an array of strings
/// TODO: Rust uses enum to represent JavaScript's string | string[] union type
/// JavaScript: field: string | string[]
#[derive(Debug, Clone, Default)]
pub enum StringOrVec {
    #[default]
    Empty,
    Single(String),
    Multiple(Vec<String>),
}

impl PartialEq<&str> for StringOrVec {
    fn eq(&self, other: &&str) -> bool {
        match self {
            StringOrVec::Empty => false,
            StringOrVec::Single(s) => s == *other,
            StringOrVec::Multiple(v) => v.iter().any(|s| s == *other),
        }
    }
}

impl PartialEq<String> for StringOrVec {
    fn eq(&self, other: &String) -> bool {
        match self {
            StringOrVec::Empty => false,
            StringOrVec::Single(s) => s == other,
            StringOrVec::Multiple(v) => v.iter().any(|s| s == other),
        }
    }
}

impl<'de> Deserialize<'de> for StringOrVec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, SeqAccess, Visitor};

        struct StringOrVecVisitor;

        impl<'de> Visitor<'de> for StringOrVecVisitor {
            type Value = StringOrVec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string or array of strings")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StringOrVec::Single(value.to_string()))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut vec = Vec::new();
                while let Some(value) = seq.next_element()? {
                    vec.push(value);
                }
                Ok(StringOrVec::Multiple(vec))
            }
        }

        deserializer.deserialize_any(StringOrVecVisitor)
    }
}

impl Serialize for StringOrVec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            StringOrVec::Empty => serializer.serialize_none(),
            StringOrVec::Single(s) => serializer.serialize_str(s),
            StringOrVec::Multiple(v) => v.serialize(serializer),
        }
    }
}

impl StringOrVec {
    pub fn as_vec(&self) -> Vec<String> {
        match self {
            StringOrVec::Empty => Vec::new(),
            StringOrVec::Single(s) => vec![s.clone()],
            StringOrVec::Multiple(v) => v.clone(),
        }
    }

    pub fn contains(&self, value: &str) -> bool {
        match self {
            StringOrVec::Empty => false,
            StringOrVec::Single(s) => s == value,
            StringOrVec::Multiple(v) => v.iter().any(|s| s == value),
        }
    }
}
