// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::pokemon::MoveSlot;

impl Pokemon {

    /// Get mutable move slot data
    // TODO: Verify move parameter type matches JavaScript's ActiveMove usage
    pub fn get_move_data_mut(&mut self, move_id: &ID) -> Option<&mut MoveSlot> {
        self.move_slots.iter_mut().find(|slot| &slot.id == move_id)
    }
}
