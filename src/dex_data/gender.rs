//! Gender type

use serde::{Deserialize, Serialize};

/// Gender names
/// JavaScript equivalent: GenderName (sim/global-types.ts)
/// JavaScript: 'M' | 'F' | 'N'
/// JavaScript uses a lookup table: {M: "M", F: "F", N: "N"}
/// Any other value (like "Male", "Female", "None") falls through to randomization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Default)]
pub enum Gender {
    Male,
    Female,
    #[default]
    None,
}

// Custom deserializer to match JavaScript's behavior:
// Only "M", "F", "N" are recognized. Everything else becomes None (needs randomization)
impl<'de> Deserialize<'de> for Gender {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "M" => Gender::Male,
            "F" => Gender::Female,
            "N" => Gender::None,
            _ => {
                // JavaScript: genders[set.gender] returns undefined for "Male", "Female", "None"
                // This triggers randomization via battle.sample(['M', 'F'])
                debug_elog!("[GENDER_DESERIALIZE] Unrecognized gender '{}', will need randomization", s);
                Gender::None
            }
        })
    }
}

impl Gender {
    pub fn parse(s: &str) -> Self {
        match s {
            "M" => Gender::Male,
            "F" => Gender::Female,
            _ => Gender::None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Gender::Male => "M",
            Gender::Female => "F",
            Gender::None => "",
        }
    }
}
