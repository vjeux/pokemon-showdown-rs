use crate::side::*;
use crate::*;

impl Side {

    /// Get a reference to a Pokemon by index
    pub fn get_pokemon(&self, index: usize) -> Option<&Pokemon> {
        self.pokemon.get(index)
    }
}
