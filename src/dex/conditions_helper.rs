// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::dex_conditions::DexConditions;

impl Dex {
    /// Get conditions helper
    /// Equivalent to accessing `dex.conditions` in TypeScript
    pub fn conditions(&self) -> DexConditions<'_> {
        DexConditions { dex: self }
    }
}
