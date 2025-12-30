use crate::*;

impl Battle {

    /// Get a Pokemon by its full name (mutable)
    /// Rust helper method - JavaScript getPokemon() can return mutable references directly
    /// Rust requires separate methods for immutable and mutable borrows
    /// Returns position tuple (side_index, pokemon_index) instead of reference for flexibility
    pub fn get_pokemon_mut(&mut self, fullname: &str) -> Option<(usize, usize)> {
        for (side_idx, side) in self.sides.iter().enumerate() {
            for (poke_idx, pokemon) in side.pokemon.iter().enumerate() {
                let poke_fullname = format!("{}: {}", side.id_str(), pokemon.name);
                if poke_fullname == fullname {
                    return Some((side_idx, poke_idx));
                }
            }
        }
        None
    }
}
