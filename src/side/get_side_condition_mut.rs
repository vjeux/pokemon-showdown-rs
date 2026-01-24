// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::side::*;
use crate::event_system::SharedEffectState;

impl Side {

    /// Get mutable side condition state
    /// Note: With SharedEffectState, callers should use .borrow_mut() on the returned value
    pub fn get_side_condition_mut(&mut self, id: &ID) -> Option<&mut SharedEffectState> {
        self.side_conditions.get_mut(id)
    }
}
