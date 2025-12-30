//! DexSpecies - Species lookup helper
//!
//! Equivalent to DexSpecies class in dex-species.ts

use crate::dex::{Dex, SpeciesData};
use crate::dex_data::ID;

/// Helper struct for species lookups
/// Equivalent to DexSpecies class in TypeScript
pub struct DexSpecies<'a> {
    pub(crate) dex: &'a Dex,
}

impl<'a> DexSpecies<'a> {
    /// Get species data by name or ID
    /// Equivalent to DexSpecies.get() in dex-species.ts
    pub fn get(&self, name: &str) -> Option<&'a SpeciesData> {
        let id = ID::new(name);
        // Try direct lookup first
        if let Some(species) = self.dex.species.get(&id) {
            return Some(species);
        }
        // Try alias lookup
        if let Some(canonical_name) = self.dex.aliases.get(&id) {
            let canonical_id = ID::new(canonical_name);
            return self.dex.species.get(&canonical_id);
        }
        None
    }

    /// Get species by ID
    /// Equivalent to DexSpecies.getByID() in dex-species.ts
    pub fn get_by_id(&self, id: &ID) -> Option<&'a SpeciesData> {
        self.dex.species.get(id)
    }

    /// Get all species data
    /// Equivalent to DexSpecies.all() in dex-species.ts
    pub fn all(&self) -> Vec<&'a SpeciesData> {
        self.dex.species.values().collect()
    }
}
