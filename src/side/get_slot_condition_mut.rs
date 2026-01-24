// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::side::*;
use crate::event_system::SharedEffectState;

impl Side {

    /// Get mutable slot condition data
    /// Note: With SharedEffectState, callers should use .borrow_mut() on the returned value
    pub fn get_slot_condition_mut(&mut self, slot: usize, id: &ID) -> Option<&mut SharedEffectState> {
        self.slot_conditions.get_mut(slot)?.get_mut(id)
    }
}
