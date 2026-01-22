// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::battle_actions::ActiveMove;

impl Pokemon {

    /// Set PP for a specific move (used by moves like Grudge)
    pub fn set_pp(&mut self, active_move: &ActiveMove, pp: u8) -> bool {
        let move_id = &active_move.id;
        if let Some(slot) = self.move_slots.iter_mut().find(|s| &s.id == move_id) {
            slot.pp = pp;
            // Also sync to base_move_slots so clearVolatile preserves PP
            // (In JS, moveSlots and baseMoveSlots share the same MoveSlot objects)
            if let Some(base_slot) = self.base_move_slots.iter_mut().find(|s| s.id == *move_id) {
                base_slot.pp = pp;
            }
            true
        } else {
            false
        }
    }
}
