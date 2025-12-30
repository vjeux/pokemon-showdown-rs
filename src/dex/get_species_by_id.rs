use crate::*;
use crate::dex::SpeciesData;

impl Dex {

    // =========================================================================
    // Species-specific methods (from dex-species.ts)
    // =========================================================================

    /// Get species by ID (equivalent to DexSpecies.getByID)
    pub fn get_species_by_id(&self, id: &ID) -> Option<&SpeciesData> {
        self.species.get(id)
    }
}
