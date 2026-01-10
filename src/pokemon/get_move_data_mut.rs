// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::pokemon::MoveSlot;
use crate::battle_actions::ActiveMove;

impl Pokemon {

    /// Get mutable move slot data
    pub fn get_move_data_mut(&mut self, active_move: &ActiveMove) -> Option<&mut MoveSlot> {
        self.move_slots.iter_mut().find(|slot| slot.id == active_move.id)
    }
}
