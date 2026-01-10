//! Accuracy type for moves

use serde::{Deserialize, Serialize};

/// Accuracy can be a number or true (always hits)
/// Rust uses enum to represent JavaScript's number | true union type
/// JavaScript: accuracy: number | true
#[derive(Debug, Clone)]
pub enum Accuracy {
    Percent(i32),
    AlwaysHits,
}

impl Default for Accuracy {
    fn default() -> Self {
        Accuracy::Percent(100)
    }
}

impl Serialize for Accuracy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Accuracy::AlwaysHits => serializer.serialize_bool(true),
            Accuracy::Percent(p) => serializer.serialize_i32(*p),
        }
    }
}

impl<'de> Deserialize<'de> for Accuracy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct AccuracyVisitor;

        impl<'de> serde::de::Visitor<'de> for AccuracyVisitor {
            type Value = Accuracy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a boolean (true) or an integer")
            }

            fn visit_bool<E>(self, value: bool) -> Result<Accuracy, E>
            where
                E: serde::de::Error,
            {
                if value {
                    Ok(Accuracy::AlwaysHits)
                } else {
                    Err(E::custom("accuracy can only be true (always hits) or a number"))
                }
            }

            fn visit_i32<E>(self, value: i32) -> Result<Accuracy, E>
            where
                E: serde::de::Error,
            {
                Ok(Accuracy::Percent(value))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Accuracy, E>
            where
                E: serde::de::Error,
            {
                Ok(Accuracy::Percent(value as i32))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Accuracy, E>
            where
                E: serde::de::Error,
            {
                Ok(Accuracy::Percent(value as i32))
            }
        }

        deserializer.deserialize_any(AccuracyVisitor)
    }
}
