
use crate::dex::IsMax;

pub fn deserialize_is_max<'de, D>(deserializer: D) -> Result<Option<IsMax>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, Visitor};

    struct IsMaxVisitor;

    impl<'de> Visitor<'de> for IsMaxVisitor {
        type Value = Option<IsMax>;

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
                Ok(Some(IsMax::Generic))
            } else {
                Err(E::custom("isMax cannot be false"))
            }
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(IsMax::Species(value.to_string())))
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(IsMax::Species(value)))
        }
    }

    deserializer.deserialize_any(IsMaxVisitor)
}
