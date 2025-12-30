use crate::*;

impl Dex {

    /// Check if a species can evolve
    pub fn can_evolve(&self, species_name: &str) -> bool {
        self.species().get(species_name)
            .map(|s| !s.evos.is_empty())
            .unwrap_or(false)
    }
}
