// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Pokemon {

    /// Get base species name for this Pokemon
    /// For formes like "Pikachu-Alola", returns "Pikachu"
    /// For base species like "Pikachu", returns "Pikachu"
    /// Equivalent to pokemon.baseSpecies.name in TypeScript
    pub fn get_base_species_name(&self, dex: &crate::dex::Dex) -> Option<String> {
        let species = dex.species().get(self.species_id.as_str())?;
        Some(
            species
                .base_species
                .clone()
                .unwrap_or_else(|| species.name.clone()),
        )
    }
}
