// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::side::*;

impl Side {

    /// Faint a Pokemon
    /// Note: Unlike a previous implementation, we do NOT clear self.active[slot] here.
    /// In JavaScript, fainted Pokemon remain in the active array until replaced by a switch.
    /// Clearing it here causes lookup failures when processing switch actions.
    pub fn faint_pokemon(&mut self, slot: usize) {
        if let Some(Some(idx)) = self.active.get(slot) {
            let idx = *idx;
            if let Some(pokemon) = self.pokemon.get_mut(idx) {
                pokemon.fainted = true;
                pokemon.faint_queued = false;
                pokemon.hp = 0;
            }
            self.fainted_this_turn = Some(idx);
            self.total_fainted += 1;
            self.pokemon_left = self.pokemon_left.saturating_sub(1);
            // DO NOT clear self.active[slot] = None here!
            // JavaScript never clears the active slot on faint - Pokemon stays until replaced by switch.
        }
    }
}
