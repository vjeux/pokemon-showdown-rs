use crate::side::*;
use crate::*;

impl Side {

    /// Check if a slot condition is active
    pub fn has_slot_condition(&self, slot: usize, id: &ID) -> bool {
        self.slot_conditions
            .get(slot)
            .map(|conds| conds.contains_key(id))
            .unwrap_or(false)
    }
}
