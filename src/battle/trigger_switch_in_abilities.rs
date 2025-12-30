use crate::*;

impl Battle {

    /// Trigger abilities that activate on switch-in
    pub fn trigger_switch_in_abilities(&mut self, side_idx: usize, poke_idx: usize) {
        // Use event system to trigger SwitchIn abilities
        // This will call handle_ability_event which handles Intimidate, Drizzle, etc.
        self.run_event("SwitchIn", Some((side_idx, poke_idx)), None, None, None);

        // TODO: Migrate all switch-in ability logic to ability_callbacks/
        // Old data-driven code removed - abilities should use event handlers instead
    }
}
