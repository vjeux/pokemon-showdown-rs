use crate::*;

impl Battle {

    /// Execute a switch with optional drag flag
    /// Rust helper - breaks down switch logic for borrow checker
    /// JavaScript integrates this into switch/drag handling
    #[allow(dead_code)]
    pub fn do_switch_with_drag(
        &mut self,
        side_idx: usize,
        slot: usize,
        switch_to: usize,
        is_drag: bool,
    ) {
        if side_idx >= self.sides.len() {
            return;
        }

        // Check if switch_to Pokemon is valid
        if switch_to >= self.sides[side_idx].pokemon.len() {
            return;
        }
        if self.sides[side_idx].pokemon[switch_to].is_fainted() {
            return;
        }
        if self.sides[side_idx].pokemon[switch_to].is_active {
            return;
        }

        // Perform the switch
        self.sides[side_idx].switch_in(slot, switch_to);

        // Log the switch/drag
        if let Some(pokemon) = self.sides[side_idx].get_active(slot) {
            let side_id = self.sides[side_idx].id_str();
            let details = pokemon.details();
            let hp = format!("{}/{}", pokemon.hp, pokemon.maxhp);
            let event = if is_drag { "drag" } else { "switch" };
            self.log.push(format!(
                "|{}|{}: {}|{}|{}",
                event, side_id, pokemon.name, details, hp
            ));
        }

        // TODO: Hazards should be applied via side condition onSwitchIn callbacks
        // (triggered by field_event_switch_in in run_switch), not directly here.

        // Trigger switch-in abilities
        self.trigger_switch_in_abilities(side_idx, switch_to);

        // In Gen 5+, run switch immediately for drags
        if is_drag && self.gen >= 5 {
            self.run_switch(side_idx, switch_to);
        }
    }
}
