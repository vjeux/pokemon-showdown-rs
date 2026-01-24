// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::event_system::SharedEffectState;

impl Pokemon {

    /// Get mutable volatile state
    /// Note: With SharedEffectState, callers should use .borrow_mut() on the returned value
    pub fn get_volatile_mut(&mut self, id: &ID) -> Option<&mut SharedEffectState> {
        self.volatiles.get_mut(id)
    }
}
