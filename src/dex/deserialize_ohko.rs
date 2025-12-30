use crate::*;

use crate::dex::Ohko;

pub fn deserialize_ohko<'de, D>(deserializer: D) -> Result<Option<Ohko>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, Visitor};

    struct OhkoVisitor;

    impl<'de> Visitor<'de> for OhkoVisitor {
        type Value = Option<Ohko>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean or string")
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
                Ok(Some(Ohko::Generic))
            } else {
                Err(E::custom("ohko cannot be false"))
            }
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(Ohko::TypeBased(value.to_string())))
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(Ohko::TypeBased(value)))
        }
    }

    deserializer.deserialize_any(OhkoVisitor)
}
