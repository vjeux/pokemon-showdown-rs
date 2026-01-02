// NOTE: This method is NOT in JavaScript - Rust-specific implementation


use crate::dex::Accuracy;

pub fn deserialize_accuracy<'de, D>(deserializer: D) -> Result<Accuracy, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, Visitor};

    struct AccuracyVisitor;

    impl<'de> Visitor<'de> for AccuracyVisitor {
        type Value = Accuracy;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a number or boolean")
        }

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value {
                Ok(Accuracy::AlwaysHits)
            } else {
                Ok(Accuracy::Percent(0))
            }
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Accuracy::Percent(value as i32))
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Accuracy::Percent(value as i32))
        }
    }

    deserializer.deserialize_any(AccuracyVisitor)
}
