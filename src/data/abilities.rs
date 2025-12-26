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
// TODO: Refactor handle_ability_event to use ability_callbacks instead
#[derive(Debug, Clone)]
pub struct AbilityDef {
    pub id: ID,
}

impl AbilityDef {
    pub fn from_id(id: ID) -> Self {
        Self { id }
    }
}

/// Temporary compatibility function for code that doesn't have access to dex
/// TODO: Refactor callers to use dex.get_ability() instead
pub fn get_ability(id: &ID) -> Option<AbilityDef> {
    use once_cell::sync::Lazy;
    use crate::dex::Dex;

    static DEX: Lazy<Dex> = Lazy::new(|| {
        Dex::load_default().expect("Failed to load default dex")
    });

    if DEX.get_ability(id.as_str()).is_some() {
        Some(AbilityDef::from_id(id.clone()))
    } else {
        None
    }
}

