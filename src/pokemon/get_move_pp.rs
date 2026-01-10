// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::battle_actions::ActiveMove;

impl Pokemon {

    /// Get move PP for a move
    pub fn get_move_pp(&self, active_move: &ActiveMove) -> Option<u8> {
        self.move_slots
            .iter()
            .find(|slot| slot.id == active_move.id)
            .map(|slot| slot.pp)
    }
}
