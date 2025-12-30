use crate::*;
use crate::dex::AbilityData;

impl Dex {

    /// Get all abilities data
    /// Equivalent to DexAbilities.all() in dex-abilities.ts
    pub fn all_abilities(&self) -> Vec<&AbilityData> {
        self.abilities.values().collect()
    }
}
