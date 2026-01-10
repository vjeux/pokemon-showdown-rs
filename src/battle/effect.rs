//! Effect

use crate::dex_data::ID;
use serde::{Deserialize, Serialize};
use super::EffectType;

/// Effect - represents an effect with its ID and type
/// JavaScript equivalent: Effect interface (sim/global-types.ts)
/// In JavaScript, Effect has many fields (id, name, effectType, flags, etc.)
/// In Rust, we only need the essential fields for singleEvent: id and effectType
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Effect {
    /// Effect ID (e.g., "stall", "intimidate", "leftovers")
    pub id: ID,
    /// Type of effect (Ability, Item, Move, Condition, etc.)
    pub effect_type: EffectType,
    /// Pokemon that holds this effect (for volatiles, abilities, items, status)
    pub effect_holder: Option<(usize, usize)>,
    /// Side index (for side conditions)
    pub side_index: Option<usize>,
    /// Whether this effect was Prankster boosted
    pub prankster_boosted: bool,
}

impl Effect {
    /// Create a new Effect with the given id and effect_type
    pub fn new(id: ID, effect_type: EffectType) -> Self {
        Self {
            id,
            effect_type,
            effect_holder: None,
            side_index: None,
            prankster_boosted: false,
        }
    }

    /// Get the Effect's ID
    pub fn get_id(&self) -> &ID {
        &self.id
    }

    /// Create an Ability effect
    pub fn ability(id: impl Into<ID>) -> Self {
        Self::new(id.into(), EffectType::Ability)
    }

    /// Create an Item effect
    pub fn item(id: impl Into<ID>) -> Self {
        Self::new(id.into(), EffectType::Item)
    }

    /// Create a Move effect
    pub fn move_(id: impl Into<ID>) -> Self {
        Self::new(id.into(), EffectType::Move)
    }

    /// Create a Condition (volatile) effect
    pub fn condition(id: impl Into<ID>) -> Self {
        Self::new(id.into(), EffectType::Condition)
    }

    /// Create a Status effect
    pub fn status(id: impl Into<ID>) -> Self {
        Self::new(id.into(), EffectType::Status)
    }

    /// Create a Weather effect
    pub fn weather(id: impl Into<ID>) -> Self {
        Self::new(id.into(), EffectType::Weather)
    }

    /// Create a Terrain effect
    pub fn terrain(id: impl Into<ID>) -> Self {
        Self::new(id.into(), EffectType::Terrain)
    }

    /// Create a SideCondition effect
    pub fn side_condition(id: impl Into<ID>) -> Self {
        Self::new(id.into(), EffectType::SideCondition)
    }

    /// Create a SlotCondition effect
    pub fn slot_condition(id: impl Into<ID>) -> Self {
        Self::new(id.into(), EffectType::SlotCondition)
    }

    /// Create a FieldCondition (pseudo-weather) effect
    pub fn field_condition(id: impl Into<ID>) -> Self {
        Self::new(id.into(), EffectType::FieldCondition)
    }

    /// Get the ID as a string reference
    pub fn as_str(&self) -> &str {
        self.id.as_str()
    }
}

impl std::fmt::Display for Effect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
