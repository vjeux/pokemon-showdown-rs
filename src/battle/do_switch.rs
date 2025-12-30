use crate::*;

impl Battle {

    /// Execute a switch
    pub fn do_switch(&mut self, side_idx: usize, slot: usize, switch_to: usize) {
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

        // JS: if (action.choice === 'switch' && action.pokemon.status) {
        //         this.singleEvent('CheckShow', this.dex.abilities.getByID('naturalcure'), null, action.pokemon);
        //     }
        // Check if the switching Pokemon has a status condition
        if let Some(active_idx) = self.sides[side_idx].active.get(slot).copied().flatten() {
            let has_status = !self.sides[side_idx].pokemon[active_idx].status.is_empty();
            if has_status {
                let naturalcure_id = ID::new("naturalcure");
                self.single_event(
                    "CheckShow",
                    &naturalcure_id,
                    Some((side_idx, active_idx)),
                    None,
                    None,
                );
            }
        }

        // Get the old Pokemon's name for logging
        let _old_name = self.sides[side_idx]
            .get_active(slot)
            .map(|p| p.name.clone())
            .unwrap_or_default();

        // Perform the switch
        self.sides[side_idx].switch_in(slot, switch_to);

        // Log the switch
        if let Some(pokemon) = self.sides[side_idx].get_active(slot) {
            let side_id = self.sides[side_idx].id_str();
            let details = pokemon.details();
            let hp = format!("{}/{}", pokemon.hp, pokemon.maxhp);
            self.log.push(format!(
                "|switch|{}: {}|{}|{}",
                side_id, pokemon.name, details, hp
            ));
        }

        // Apply entry hazard damage
        self.apply_hazards(side_idx, slot, switch_to);

        // Trigger switch-in abilities
        self.trigger_switch_in_abilities(side_idx, switch_to);
    }
}
