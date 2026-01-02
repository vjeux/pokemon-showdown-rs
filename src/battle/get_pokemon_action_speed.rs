// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::dex_data::StatID;

impl Battle {

    /// Get a Pokemon's action speed (helper that delegates to Pokemon::get_action_speed)
    /// This is a Rust helper for the borrow checker - allows calling pokemon.getActionSpeed(battle)
    /// when we only have indices.
    pub fn get_pokemon_action_speed(&mut self, side_idx: usize, poke_idx: usize) -> i32 {
        if let Some(_pokemon) = self.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
            self.get_pokemon_stat((side_idx, poke_idx), StatID::Spe, false, false)
        } else {
            0
        }
    }
}
