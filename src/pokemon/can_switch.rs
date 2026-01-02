// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Pokemon {

    /// Check if Pokemon can switch out
    pub fn can_switch(&self) -> bool {
        !self.trapped && !self.fainted
    }
}
