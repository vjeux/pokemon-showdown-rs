use crate::*;
use crate::event_system::EffectState;

impl Pokemon {

    /// Get mutable volatile state
    pub fn get_volatile_mut(&mut self, id: &ID) -> Option<&mut EffectState> {
        self.volatiles.get_mut(id)
    }
}
