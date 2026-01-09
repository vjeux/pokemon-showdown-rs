// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::battle_actions::Damage;

/// Deserialize damage which can be:
/// - number -> Damage::Fixed(n)
/// - "level" -> Damage::Level
/// - false or null -> None
pub fn deserialize_damage<'de, D>(deserializer: D) -> Result<Option<Damage>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, Visitor};

    struct DamageVisitor;

    impl<'de> Visitor<'de> for DamageVisitor {
        type Value = Option<Damage>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a number, 'level' string, or false/null")
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
                // true is not a valid damage value, treat as error
                Err(E::custom("damage cannot be true"))
            } else {
                // false means no fixed damage
                Ok(None)
            }
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(Damage::Fixed(value as i32)))
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(Damage::Fixed(value as i32)))
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value == "level" {
                Ok(Some(Damage::Level))
            } else {
                Err(E::custom(format!("unknown damage string: {}", value)))
            }
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_str(&value)
        }
    }

    deserializer.deserialize_any(DamageVisitor)
}
