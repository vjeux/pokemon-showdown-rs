// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::side::*;

impl Side {

    /// Get a reference to a Pokemon by index
    pub fn get_pokemon(&self, index: usize) -> Option<&Pokemon> {
        self.pokemon.get(index)
    }
}
