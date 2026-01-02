// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::dex_items::DexItems;

impl Dex {
    /// Get items helper
    /// Equivalent to accessing `dex.items` in TypeScript
    pub fn items(&self) -> DexItems<'_> {
        DexItems { dex: self }
    }
}
