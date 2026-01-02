// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Pokemon {

    /// Clear all volatile conditions
    pub fn clear_volatiles(&mut self) {
        self.volatiles.clear();
    }
}
