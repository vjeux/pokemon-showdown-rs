use crate::*;
use crate::dex_formats::DexFormats;

impl Dex {
    /// Get formats helper
    /// Equivalent to accessing `dex.formats` in TypeScript
    pub fn formats(&self) -> DexFormats<'_> {
        DexFormats { dex: self }
    }
}
