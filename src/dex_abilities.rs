//! DexAbilities - Abilities lookup helper
//!
//! Equivalent to DexAbilities class in dex-abilities.ts

use crate::dex::{Dex, AbilityData};
use crate::dex_data::ID;

/// Helper struct for abilities lookups
/// Equivalent to DexAbilities class in TypeScript
/// JavaScript equivalent: DexAbilities (sim/dex-abilities.ts)
/// Fields: dex, allCache, name
/// JavaScript equivalent: DexAbilities (sim/dex-abilities.ts)
/// Fields: dex, allCache, name
/// JavaScript equivalent: DexAbilities (sim/dex-abilities.ts)
/// Fields: dex, allCache, name
pub struct DexAbilities<'a> {
    pub(crate) dex: &'a Dex,
}

impl<'a> DexAbilities<'a> {
    /// Get ability data by name or ID
    /// Equivalent to DexAbilities.get() in dex-abilities.ts
    pub fn get(&self, name: &str) -> Option<&'a AbilityData> {
        let id = ID::new(name);
        // Try direct lookup first
        if let Some(ability) = self.dex.abilities.get(&id) {
            return Some(ability);
        }
        // Try alias lookup
        if let Some(canonical_name) = self.dex.aliases.get(&id) {
            let canonical_id = ID::new(canonical_name);
            return self.dex.abilities.get(&canonical_id);
        }
        None
    }

    /// Get ability by ID
    /// Equivalent to DexAbilities.getByID() in dex-abilities.ts
    pub fn get_by_id(&self, id: &ID) -> Option<&'a AbilityData> {
        self.dex.abilities.get(id)
    }

    /// Get all abilities data
    /// Equivalent to DexAbilities.all() in dex-abilities.ts
    pub fn all(&self) -> Vec<&'a AbilityData> {
        self.dex.abilities.values().collect()
    }
}
