use crate::*;
use crate::dex_types::DexTypes;

impl Dex {
    /// Get types helper
    /// Equivalent to accessing `dex.types` in TypeScript
    pub fn types(&self) -> DexTypes<'_> {
        DexTypes { dex: self }
    }
}
