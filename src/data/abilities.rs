// All abilities are now loaded from data/abilities.json
// This file is kept for compatibility with existing code that references ability constants
//
// Ability data comes from:
// 1. data/abilities.json - base ability data (name, desc, rating, etc.)
// 2. src/data/ability_callbacks/ - custom event handlers per ability
//
// Access abilities via Battle.dex.get_ability(name) or Battle.dex.abilities

use crate::dex_data::ID;

// Re-export ability data type from dex
pub use crate::dex::AbilityData;

// Minimal compatibility struct for existing code
#[derive(Debug, Clone)]
pub struct AbilityDef {
    pub id: ID,
}

impl AbilityDef {
    pub fn from_id(id: ID) -> Self {
        Self { id }
    }
}

