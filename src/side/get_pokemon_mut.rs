// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::side::*;

impl Side {

    /// Get a mutable reference to a Pokemon by index
    pub fn get_pokemon_mut(&mut self, index: usize) -> Option<&mut Pokemon> {
        self.pokemon.get_mut(index)
    }
}
