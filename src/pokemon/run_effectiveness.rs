use crate::*;

impl Pokemon {

    /// Run type effectiveness check
    /// Equivalent to runEffectiveness in pokemon.ts
    //
    // 	runEffectiveness(move: ActiveMove) {
    // 		let totalTypeMod = 0;
    // 		if (this.terastallized && move.type === 'Stellar') {
    // 			totalTypeMod = 1;
    // 		} else {
    // 			for (const type of this.getTypes()) {
    // 				let typeMod = this.battle.dex.getEffectiveness(move, type);
    // 				typeMod = this.battle.singleEvent('Effectiveness', move, null, this, type, move, typeMod);
    // 				totalTypeMod += this.battle.runEvent('Effectiveness', this, type, move, typeMod);
    // 			}
    // 		}
    // 		if (this.species.name === 'Terapagos-Terastal' && this.hasAbility('Tera Shell') &&
    // 			!this.battle.suppressingAbility(this)) {
    // 			if (this.abilityState.resisted) return -1; // all hits of multi-hit move should be not very effective
    // 			if (move.category === 'Status' || move.id === 'struggle' || !this.runImmunity(move) ||
    // 				totalTypeMod < 0 || this.hp < this.maxhp) {
    // 				return totalTypeMod;
    // 			}
    //
    // 			this.battle.add('-activate', this, 'ability: Tera Shell');
    // 			this.abilityState.resisted = true;
    // 			return -1;
    // 		}
    // 		return totalTypeMod;
    // 	}
    //
    /// Refactored to associated function to access Battle for event calls
    /// Call as: Pokemon::run_effectiveness(battle, pokemon_pos, move_id)
    pub fn run_effectiveness(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        move_id: &ID,
    ) -> i32 {
        // JS: let totalTypeMod = 0;
        let mut total_type_mod = 0;

        // Get move type
        let move_type = if let Some(active_move) = &battle.active_move {
            active_move.move_type.clone()
        } else if let Some(move_data) = battle.dex.moves().get_by_id(move_id) {
            move_data.move_type.clone()
        } else {
            return total_type_mod;
        };

        // JS: if (this.terastallized && move.type === 'Stellar') {
        // JS:     totalTypeMod = 1;
        // JS: } else {
        let (is_terastallized, pokemon_types) = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return total_type_mod,
            };
            (pokemon.terastallized.is_some(), pokemon.get_types(battle, false))
        };

        if is_terastallized && move_type == "Stellar" {
            total_type_mod = 1;
        } else {
            // JS: for (const type of this.getTypes()) {
            for defender_type in &pokemon_types {
                // JS: let typeMod = this.battle.dex.getEffectiveness(move, type);
                let mut type_mod = battle.dex.get_effectiveness(&move_type, defender_type);

                // JS: typeMod = this.battle.singleEvent('Effectiveness', move, null, this, type, move, typeMod);
                // ✅ NOW IMPLEMENTED (Session 24 Part 92): singleEvent with relay_var support
                // Call singleEvent on the move to allow move-specific effectiveness modifiers (e.g., Freeze-Dry)
                let single_event_result = battle.single_event_with_relay_var(
                    "Effectiveness",
                    move_id,
                    Some(pokemon_pos),
                    None,
                    None,
                    Some(type_mod),
                );

                // Extract modified type_mod from event result
                if let crate::event::EventResult::Number(modified_mod) = single_event_result {
                    type_mod = modified_mod;
                }

                // JS: totalTypeMod += this.battle.runEvent('Effectiveness', this, type, move, typeMod);
                // runEvent returns modified effectiveness or None if event fails
                let event_result = battle.run_event(
                    "Effectiveness",
                    Some(pokemon_pos),
                    None,
                    Some(move_id),
                    Some(type_mod),
                );

                if let Some(modified_mod) = event_result {
                    total_type_mod += modified_mod;
                } else {
                    // Event didn't modify, use base type_mod
                    total_type_mod += type_mod;
                }
            }
        }

        // JS: if (this.species.name === 'Terapagos-Terastal' && this.hasAbility('Tera Shell') &&
        // JS:     !this.battle.suppressingAbility(this)) {
        let (is_terapagos_terastal, has_tera_shell, ability_state_resisted, is_suppressed, hp, maxhp) = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return total_type_mod,
            };

            // Check species_id for Terapagos-Terastal
            let is_terapagos_terastal = pokemon.species_id.as_str() == "terapagosterastal";
            let has_tera_shell = pokemon.has_ability(battle, &["Tera Shell"]);
            let ability_state_resisted = pokemon
                .ability_state
                .data
                .get("resisted")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            // Check if ability is suppressed
            let is_suppressed = battle.suppressing_ability(Some(pokemon_pos));

            (is_terapagos_terastal, has_tera_shell, ability_state_resisted, is_suppressed, pokemon.hp, pokemon.maxhp)
        };

        if is_terapagos_terastal && has_tera_shell && !is_suppressed {
            // JS: if (this.abilityState.resisted) return -1;
            if ability_state_resisted {
                return -1; // all hits of multi-hit move should be not very effective
            }

            // Get move category and id
            let (move_category, move_id_str) = if let Some(active_move) = &battle.active_move {
                (active_move.category.clone(), active_move.id.as_str().to_string())
            } else if let Some(move_data) = battle.dex.moves().get_by_id(move_id) {
                (move_data.category.clone(), move_id.as_str().to_string())
            } else {
                return total_type_mod;
            };

            // JS: if (move.category === 'Status' || move.id === 'struggle' || !this.runImmunity(move) ||
            // JS:     totalTypeMod < 0 || this.hp < this.maxhp) {
            // JS:     return totalTypeMod;
            // JS: }

            // Check if move is Status category
            if move_category == "Status" {
                return total_type_mod;
            }

            // Check if move is struggle
            if move_id_str == "struggle" {
                return total_type_mod;
            }

            // Check if Pokemon is immune to the move type
            let has_immunity = Pokemon::run_immunity(battle, pokemon_pos, &move_type, false);
            if !has_immunity {
                return total_type_mod;
            }

            // Check if effectiveness is already negative
            if total_type_mod < 0 {
                return total_type_mod;
            }

            // Check if HP is less than max HP
            if hp < maxhp {
                return total_type_mod;
            }

            // JS: this.battle.add('-activate', this, 'ability: Tera Shell');
            let pokemon_ident = {
                let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => return total_type_mod,
                };
                pokemon.get_slot()
            };

            battle.add("-activate", &[
                pokemon_ident.as_str().into(),
                "ability: Tera Shell".into(),
            ]);

            // JS: this.abilityState.resisted = true;
            if let Some(pokemon_mut) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                pokemon_mut.ability_state.data.insert(
                    "resisted".to_string(),
                    serde_json::json!(true),
                );
            }

            // JS: return -1;
            return -1;
        }

        // JS: return totalTypeMod;
        total_type_mod
    }
}
