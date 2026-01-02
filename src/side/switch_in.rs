// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::side::*;

impl Side {

    /// Switch a Pokemon into a slot
    pub fn switch_in(&mut self, slot: usize, pokemon_index: usize) -> bool {
        if slot >= self.active.len() || pokemon_index >= self.pokemon.len() {
            return false;
        }

        // Switch out current occupant
        if let Some(old_idx) = self.active[slot] {
            if let Some(old_pokemon) = self.pokemon.get_mut(old_idx) {
                old_pokemon.clear_switch_state();
            }
        }

        // Switch in new Pokemon
        self.active[slot] = Some(pokemon_index);
        if let Some(pokemon) = self.pokemon.get_mut(pokemon_index) {
            pokemon.is_active = true;
            pokemon.position = slot;
            pokemon.newly_switched = true;
            pokemon.previously_switched_in += 1;
        }

        true
    }
}
