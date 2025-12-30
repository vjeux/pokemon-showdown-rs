use crate::*;
use crate::side::Side;

impl Battle {

    /// Apply entry hazard damage to a Pokemon that just switched in
    pub fn apply_hazards(&mut self, side_idx: usize, _slot: usize, poke_idx: usize) {
        // Get Pokemon data before hazard checks
        let (maxhp, pokemon_types, pokemon_name, is_grounded) = {
            let pokemon = &self.sides[side_idx].pokemon[poke_idx];
            let is_grounded = !pokemon.types.iter().any(|t| t.to_lowercase() == "flying");
            (
                pokemon.maxhp,
                pokemon.types.clone(),
                pokemon.name.clone(),
                is_grounded,
            )
        };

        let side_id = self.sides[side_idx].id_str();
        let full_name = format!("{}: {}", side_id, pokemon_name);

        // Stealth Rock - affects all Pokemon, damage based on Rock type effectiveness
        let sr_id = ID::new("stealthrock");
        if self.sides[side_idx].has_side_condition(&sr_id) {
            let damage = Side::calc_stealth_rock_damage(&pokemon_types, maxhp);
            self.sides[side_idx].pokemon[poke_idx].take_damage(damage);

            let hp = self.sides[side_idx].pokemon[poke_idx].hp;
            self.add(
                "-damage",
                &[
                    full_name.as_str().into(),
                    format!("{}/{}", hp, maxhp).into(),
                    "[from] Stealth Rock".into(),
                ],
            );
        }

        // Spikes - only affects grounded Pokemon
        let spikes_id = ID::new("spikes");
        if is_grounded && self.sides[side_idx].has_side_condition(&spikes_id) {
            let layers = self.sides[side_idx].get_hazard_layers(&spikes_id);
            if layers > 0 {
                let damage = Side::calc_spikes_damage(layers, maxhp);
                self.sides[side_idx].pokemon[poke_idx].take_damage(damage);

                let hp = self.sides[side_idx].pokemon[poke_idx].hp;
                self.add(
                    "-damage",
                    &[full_name.as_str().into(), format!("{}/{}", hp, maxhp).into(), "[from] Spikes".into()],
                );
            }
        }

        // Toxic Spikes - only affects grounded Pokemon, poisons them
        let tspikes_id = ID::new("toxicspikes");
        if is_grounded && self.sides[side_idx].has_side_condition(&tspikes_id) {
            let layers = self.sides[side_idx].get_hazard_layers(&tspikes_id);

            // Poison types absorb Toxic Spikes
            let is_poison = pokemon_types.iter().any(|t| t.to_lowercase() == "poison");
            if is_poison {
                // Poison type absorbs Toxic Spikes
                self.sides[side_idx].remove_side_condition(&tspikes_id);
                self.add(
                    "-sideend",
                    &[side_id.into(), "Toxic Spikes".into(), "[from] ability: Poison Touch".into()],
                ); // Generic message
            } else if !self.sides[side_idx].pokemon[poke_idx].status.is_empty() {
                // Already has a status - can't be poisoned
            } else {
                // Apply poison (1 layer) or toxic (2 layers)
                let status = if layers >= 2 { "tox" } else { "psn" };
                self.sides[side_idx].pokemon[poke_idx].set_status(ID::new(status));

                let _status_msg = if layers >= 2 {
                    "badly poisoned"
                } else {
                    "poisoned"
                };
                self.add("-status", &[full_name.as_str().into(), status.into(), "[from] Toxic Spikes".into()]);
            }
        }

        // Sticky Web - lowers Speed by 1 stage (only affects grounded Pokemon)
        let sticky_web_id = ID::new("stickyweb");
        if is_grounded && self.sides[side_idx].has_side_condition(&sticky_web_id) {
            let current_spe = self.sides[side_idx].pokemon[poke_idx].boosts.spe;
            if current_spe > -6 {
                self.sides[side_idx].pokemon[poke_idx].boosts.spe = (current_spe - 1).max(-6);
                self.add("-boost", &[full_name.as_str().into(), "spe".into(), "-1".into(), "[from] Sticky Web".into()]);
            }
        }
    }
}
