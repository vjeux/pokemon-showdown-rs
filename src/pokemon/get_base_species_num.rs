use crate::*;

impl Pokemon {

    /// Get base species num for this Pokemon
    /// Equivalent to pokemon.baseSpecies.num in TypeScript
    pub fn get_base_species_num(&self, dex: &crate::dex::Dex) -> Option<i32> {
        let species = dex.species().get(self.species_id.as_str())?;
        let base_species_name = species.base_species.as_ref().unwrap_or(&species.name);
        let base_species = dex.species().get(base_species_name)?;
        Some(base_species.num)
    }
}
