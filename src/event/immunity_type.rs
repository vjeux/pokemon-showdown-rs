//! Immunity Type

use serde::{Deserialize, Serialize};

/// Categories of type immunity
/// TODO: Not in JavaScript - Rust-specific enum for categorizing immunity types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImmunityType {
    /// Type-based immunity (e.g., Ground moves on Flying types)
    Type(String),
    /// Status immunity (e.g., Electric can't be paralyzed)
    Status(String),
    /// Ability-granted immunity (e.g., Levitate)
    Ability(String),
    /// Weather immunity (e.g., Sand immunity for Rock/Ground/Steel)
    Weather(String),
    /// Terrain immunity (e.g., Psychic Terrain blocks priority)
    Terrain(String),
}
