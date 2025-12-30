use crate::*;
use crate::dex_natures::DexNatures;

impl Dex {
    /// Get natures helper
    /// Equivalent to accessing `dex.natures` in TypeScript
    pub fn natures(&self) -> DexNatures<'_> {
        DexNatures { dex: self }
    }
}
