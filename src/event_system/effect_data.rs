//! Effect Data

use super::EffectType;

/// Effect metadata - represents an effect with its properties
/// Equivalent to Effect interface in TypeScript
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// JavaScript equivalent: EffectData (sim/global-types.ts)
/// 8 fields in JavaScript
pub struct EffectData {
    /// Effect name/ID
    pub name: String,
    /// Effect type (Ability, Move, Item, etc.)
    pub effect_type: EffectType,
    /// Whether this effect was Prankster boosted
    pub prankster_boosted: bool,
}

impl EffectData {
    pub fn new(name: String, effect_type: EffectType) -> Self {
        Self {
            name,
            effect_type,
            prankster_boosted: false,
        }
    }

    pub fn with_prankster(mut self, prankster_boosted: bool) -> Self {
        self.prankster_boosted = prankster_boosted;
        self
    }
}
