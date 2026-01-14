// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Pokemon {

    /// Check if this Pokemon is fainted
    /// NOTE: This only checks the `fainted` flag, NOT HP.
    /// In JavaScript, a Pokemon with HP 0 but fainted=false can still execute its queued move.
    /// The fainted flag is set when the faint is actually processed.
    pub fn is_fainted(&self) -> bool {
        self.fainted
    }
}
