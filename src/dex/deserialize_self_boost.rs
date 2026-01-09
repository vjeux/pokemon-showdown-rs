// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::dex_data::BoostsTable;

/// Deserialize selfBoost which has structure { boosts: { atk: 1, ... } }
/// Extracts the inner boosts table directly
pub fn deserialize_self_boost<'de, D>(deserializer: D) -> Result<Option<BoostsTable>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, Visitor, MapAccess};

    struct SelfBoostVisitor;

    impl<'de> Visitor<'de> for SelfBoostVisitor {
        type Value = Option<BoostsTable>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an object with optional 'boosts' field")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_any(self)
        }

        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut boosts: Option<BoostsTable> = None;

            while let Some(key) = map.next_key::<String>()? {
                if key == "boosts" {
                    boosts = map.next_value()?;
                } else {
                    // Ignore other fields
                    let _: serde_json::Value = map.next_value()?;
                }
            }

            Ok(boosts)
        }
    }

    deserializer.deserialize_any(SelfBoostVisitor)
}
