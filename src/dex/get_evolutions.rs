use crate::*;

impl Dex {

    /// Get evolutions for a species
    pub fn get_evolutions(&self, species_name: &str) -> Vec<String> {
        self.get_species(species_name)
            .map(|s| s.evos.clone())
            .unwrap_or_default()
    }
}
