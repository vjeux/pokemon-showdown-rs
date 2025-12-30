use crate::*;
use crate::dex_species::DexSpecies;

impl Dex {
    /// Get species helper
    /// Equivalent to accessing `dex.species` in TypeScript
    pub fn species(&self) -> DexSpecies<'_> {
        DexSpecies { dex: self }
    }
}
