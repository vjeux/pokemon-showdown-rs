// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::side::*;

impl Side {

    /// Get a mutable reference to the active Pokemon in a slot
    pub fn get_active_mut(&mut self, slot: usize) -> Option<&mut Pokemon> {
        if let Some(Some(idx)) = self.active.get(slot) {
            self.pokemon.get_mut(*idx)
        } else {
            None
        }
    }
}
