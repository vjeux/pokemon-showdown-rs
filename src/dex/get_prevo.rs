use crate::*;

impl Dex {

    /// Get the pre-evolution for a species
    pub fn get_prevo(&self, species_name: &str) -> Option<String> {
        self.species().get(species_name).and_then(|s| s.prevo.clone())
    }
}
