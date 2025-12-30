use crate::side::*;

impl Side {

    /// Get mutable side condition state
    pub fn get_side_condition_mut(&mut self, id: &ID) -> Option<&mut EffectState> {
        self.side_conditions.get_mut(id)
    }
}
