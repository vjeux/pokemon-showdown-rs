use crate::*;

impl Battle {

    /// Run each event for all active Pokemon
    pub fn each_event_internal(&mut self, event_name: &str) {
        let active: Vec<(usize, usize)> = self.get_all_active(false);
        for (side_idx, poke_idx) in active {
            let effect_id = ID::new(event_name);
            self.single_event(
                event_name,
                &effect_id,
                Some((side_idx, poke_idx)),
                None,
                None,
            );
        }
    }
}
