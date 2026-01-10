// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::battle_actions::ActiveMove;

impl Pokemon {

    /// Set PP for a specific move (used by moves like Grudge)
    pub fn set_pp(&mut self, active_move: &ActiveMove, pp: u8) -> bool {
        let move_id = &active_move.id;
        if let Some(slot) = self.move_slots.iter_mut().find(|s| &s.id == move_id) {
            slot.pp = pp;
            true
        } else {
            false
        }
    }
}
