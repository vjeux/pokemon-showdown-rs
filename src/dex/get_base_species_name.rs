use crate::*;

impl Dex {

    /// Get the base species name for a forme
    /// Equivalent to species.baseSpecies access
    pub fn get_base_species_name(&self, species_name: &str) -> Option<String> {
        self.get_species(species_name)
            .map(|s| s.base_species.clone().unwrap_or_else(|| s.name.clone()))
    }
}
