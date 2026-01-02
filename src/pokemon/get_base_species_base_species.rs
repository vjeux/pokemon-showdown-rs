// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Pokemon {

    /// Get base species of base species for this Pokemon
    /// For complex formes like "Gengar-Mega", returns "Gengar"
    /// Equivalent to pokemon.baseSpecies.baseSpecies in TypeScript
    pub fn get_base_species_base_species(&self, dex: &crate::dex::Dex) -> Option<String> {
        let species = dex.species().get(self.species_id.as_str())?;
        let base_species_name = species.base_species.as_ref().unwrap_or(&species.name);
        let base_species = dex.species().get(base_species_name)?;
        Some(
            base_species
                .base_species
                .clone()
                .unwrap_or_else(|| base_species.name.clone()),
        )
    }
}
