// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::battle_actions::IgnoreImmunity;
use std::collections::HashMap;

/// Deserialize ignoreImmunity which can be:
/// - boolean true -> IgnoreImmunity::All
/// - boolean false -> IgnoreImmunity::NoIgnore (explicitly don't ignore)
/// - object { Type: true, ... } -> IgnoreImmunity::Specific
/// - missing -> None (will use default behavior based on category)
pub fn deserialize_ignore_immunity<'de, D>(deserializer: D) -> Result<Option<IgnoreImmunity>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, Visitor, MapAccess};

    struct IgnoreImmunityVisitor;

    impl<'de> Visitor<'de> for IgnoreImmunityVisitor {
        type Value = Option<IgnoreImmunity>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean or object")
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

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value {
                Ok(Some(IgnoreImmunity::All))
            } else {
                // Explicitly set to false - use NoIgnore to distinguish from undefined
                Ok(Some(IgnoreImmunity::NoIgnore))
            }
        }

        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut type_map: HashMap<String, bool> = HashMap::new();

            while let Some((key, value)) = map.next_entry::<String, bool>()? {
                type_map.insert(key, value);
            }

            Ok(Some(IgnoreImmunity::Specific(type_map)))
        }
    }

    deserializer.deserialize_any(IgnoreImmunityVisitor)
}
