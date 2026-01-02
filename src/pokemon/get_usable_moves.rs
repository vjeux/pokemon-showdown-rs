// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::pokemon::MoveSlot;

impl Pokemon {

    /// Get usable moves (not disabled, has PP)
    pub fn get_usable_moves(&self) -> Vec<&MoveSlot> {
        self.move_slots
            .iter()
            .filter(|slot| !slot.disabled && slot.pp > 0)
            .collect()
    }
}
