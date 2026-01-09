// NOTE: This method is NOT in JavaScript - Rust-specific implementation

/// Deserialize selfSwitch which can be boolean or string
/// - true -> Some("true")
/// - "copyvolatile" -> Some("copyvolatile")
/// - "shedtail" -> Some("shedtail")
/// - false or missing -> None
pub fn deserialize_self_switch<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, Visitor};

    struct SelfSwitchVisitor;

    impl<'de> Visitor<'de> for SelfSwitchVisitor {
        type Value = Option<String>;

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
                Ok(Some("true".to_string()))
            } else {
                Ok(None)
            }
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(value.to_string()))
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(value))
        }
    }

    deserializer.deserialize_any(SelfSwitchVisitor)
}
