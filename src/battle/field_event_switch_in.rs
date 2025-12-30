use crate::*;

impl Battle {

    /// Run field event for switch-in (Intimidate, etc.)
    pub fn field_event_switch_in(&mut self, switchers: &[(usize, usize)]) {
        // Run SwitchIn event for each switcher
        for (s_idx, p_idx) in switchers {
            let effect_id = ID::new("switchin");
            self.single_event("SwitchIn", &effect_id, Some((*s_idx, *p_idx)), None, None);
        }
    }
}
