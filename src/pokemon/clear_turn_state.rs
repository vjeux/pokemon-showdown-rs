// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Pokemon {

    /// Reset for a new turn
    pub fn clear_turn_state(&mut self) {
        self.move_last_turn_result = self.move_this_turn_result;
        self.move_this_turn = None;
        self.move_this_turn_result = None;
        self.hurt_this_turn = None;
        self.used_item_this_turn = false;
    }
}
