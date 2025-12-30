use crate::*;

impl Pokemon {

    /// Enable all disabled moves
    pub fn enable_moves(&mut self) {
        for slot in &mut self.move_slots {
            slot.disabled = false;
            slot.disabled_source = None;
        }
    }
}
