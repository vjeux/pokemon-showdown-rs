// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::dex_abilities::DexAbilities;

impl Dex {
    /// Get abilities helper
    /// Equivalent to accessing `dex.abilities` in TypeScript
    pub fn abilities(&self) -> DexAbilities<'_> {
        DexAbilities { dex: self }
    }
}
