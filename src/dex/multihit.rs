//! Multihit type for multi-hit moves

use serde::{Deserialize, Serialize};

/// Multihit can be a single number or range [min, max]
/// TODO: Rust uses enum to represent JavaScript's number | [number, number] union type
/// JavaScript: multihit?: number | [number, number]
#[derive(Debug, Clone)]
pub enum Multihit {
    Fixed(i32),
    Range(i32, i32),
}

impl<'de> Deserialize<'de> for Multihit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, SeqAccess, Visitor};

        struct MultihitVisitor;

        impl<'de> Visitor<'de> for MultihitVisitor {
            type Value = Multihit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a number or array of two numbers")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Multihit::Fixed(value as i32))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let min: i32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let max: i32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(Multihit::Range(min, max))
            }
        }

        deserializer.deserialize_any(MultihitVisitor)
    }
}

impl Serialize for Multihit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Multihit::Fixed(n) => serializer.serialize_i32(*n),
            Multihit::Range(min, max) => {
                use serde::ser::SerializeSeq;
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element(min)?;
                seq.serialize_element(max)?;
                seq.end()
            }
        }
    }
}
