use crate::*;

impl Battle {

    /// Run field event for switch-in (Intimidate, etc.)
    /// JavaScript: this.battle.fieldEvent('SwitchIn', switchersIn);
    /// This calls the main fieldEvent with 'SwitchIn' event and the switchers as targets
    pub fn field_event_switch_in(&mut self, switchers: &[(usize, usize)]) {
        // JS: this.battle.fieldEvent('SwitchIn', switchersIn);
        // This collects handlers from all active Pokemon (for onAnySwitchIn) and
        // from switchers only (for other handlers), then sorts by speed and executes
        self.field_event("SwitchIn", Some(switchers));
    }
}
