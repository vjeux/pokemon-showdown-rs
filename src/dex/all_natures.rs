use crate::*;
use crate::dex::NatureData;

impl Dex {

    /// Get all natures data
    /// Equivalent to DexNatures.all() in dex-data.ts
    pub fn all_natures(&self) -> Vec<&NatureData> {
        self.natures.values().collect()
    }
}
