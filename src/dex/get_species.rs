use crate::*;
use crate::dex::SpeciesData;

impl Dex {

    /// Get species data by name or ID
    pub fn get_species(&self, name: &str) -> Option<&SpeciesData> {
        let id = ID::new(name);
        // Try direct lookup first
        if let Some(species) = self.species.get(&id) {
            return Some(species);
        }
        // Try alias lookup
        if let Some(canonical_name) = self.aliases.get(&id) {
            let canonical_id = ID::new(canonical_name);
            return self.species.get(&canonical_id);
        }
        None
    }
}
