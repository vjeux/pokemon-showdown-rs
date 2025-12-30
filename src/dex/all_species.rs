use crate::*;
use crate::dex::SpeciesData;

impl Dex {

    // =========================================================================
    // Collection methods (equivalent to DexAbilities.all(), DexMoves.all(), etc.)
    // =========================================================================

    /// Get all species data
    /// Equivalent to DexSpecies.all() in dex-species.ts
    pub fn all_species(&self) -> Vec<&SpeciesData> {
        self.species.values().collect()
    }
}
