use crate::side::*;
use crate::*;

impl Side {

    /// Switch out the Pokemon in a slot
    pub fn switch_out(&mut self, slot: usize) -> Option<usize> {
        if slot >= self.active.len() {
            return None;
        }

        let old_idx = self.active[slot].take()?;
        if let Some(pokemon) = self.pokemon.get_mut(old_idx) {
            pokemon.clear_switch_state();
        }
        Some(old_idx)
    }
}
