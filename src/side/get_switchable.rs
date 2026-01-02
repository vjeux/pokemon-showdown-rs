// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::side::*;

impl Side {

    /// Get Pokemon that can switch in
    pub fn get_switchable(&self) -> Vec<usize> {
        self.pokemon
            .iter()
            .enumerate()
            .filter(|(_, p)| !p.is_fainted() && !p.is_active)
            .map(|(i, _)| i)
            .collect()
    }
}
