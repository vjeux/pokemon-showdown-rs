use crate::side::*;

impl Side {

    /// Get mutable slot condition data
    pub fn get_slot_condition_mut(&mut self, slot: usize, id: &ID) -> Option<&mut EffectState> {
        self.slot_conditions.get_mut(slot)?.get_mut(id)
    }
}
