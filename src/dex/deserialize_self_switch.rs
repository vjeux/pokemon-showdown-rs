// NOTE: This method is NOT in JavaScript - Rust-specific implementation

/// Deserialize selfSwitch which can be boolean or string
/// - true -> Some("true") (switch happens but copyVolatileFrom is NOT called)
/// - "copyvolatile" -> Some("copyvolatile") (switch + copy volatiles/boosts)
/// - "shedtail" -> Some("shedtail") (switch + copy substitute only)
/// - false or missing -> None (no switch)
///
/// IMPORTANT: JavaScript checks `typeof selfSwitch === 'string'` before calling copyVolatileFrom.
/// So boolean `true` (like U-Turn) causes a switch but does NOT copy volatiles/boosts.
/// The switch_in.rs code handles this by excluding "true" when checking switch_copy_flag.
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
            // Convert boolean true to Some("true") so we can distinguish it from string values
            // in switch_in.rs. The switch_in code will only call copyVolatileFrom for
            // specific string values ("copyvolatile", "shedtail"), NOT for "true".
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
