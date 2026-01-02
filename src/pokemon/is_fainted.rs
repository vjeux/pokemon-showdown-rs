// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Pokemon {

    /// Check if this Pokemon is fainted
    pub fn is_fainted(&self) -> bool {
        self.fainted || self.hp == 0
    }
}
