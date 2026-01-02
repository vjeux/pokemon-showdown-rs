// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Pokemon {

    /// Get forme name for this Pokemon
    /// For "Pikachu-Alola", returns Some("Alola")
    /// For "Pikachu", returns None
    /// Equivalent to pokemon.baseSpecies.forme or pokemon.species.forme in TypeScript
    pub fn get_forme(&self, dex: &crate::dex::Dex) -> Option<String> {
        let species = dex.species().get(self.species_id.as_str())?;
        species.forme.clone()
    }
}
