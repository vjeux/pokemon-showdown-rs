use crate::*;

impl Dex {

    /// Check if a species is an alternate forme
    pub fn is_alternate_forme(&self, species_name: &str) -> bool {
        self.get_species(species_name)
            .map(|s| s.forme.is_some())
            .unwrap_or(false)
    }
}
