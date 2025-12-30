use crate::*;
use crate::dex::SpeciesData;

impl Dex {

    /// Get all formes for a base species
    pub fn get_all_formes(&self, base_species: &str) -> Vec<&SpeciesData> {
        let base_id = ID::new(base_species);
        self.species
            .values()
            .filter(|s| {
                let species_base = s
                    .base_species
                    .as_ref()
                    .map(|b| ID::new(b))
                    .unwrap_or_else(|| ID::new(&s.name));
                species_base == base_id
            })
            .collect()
    }
}
