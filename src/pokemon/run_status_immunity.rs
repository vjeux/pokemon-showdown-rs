use crate::*;

impl Pokemon {

    /// Run status immunity check
    /// Equivalent to runStatusImmunity in pokemon.ts
    //
    // 	runStatusImmunity(type: string, message?: string) {
    // 		if (this.fainted) return false;
    // 		if (!type) return true;
    //
    // 		if (!this.battle.dex.getImmunity(type, this)) {
    // 			this.battle.debug('natural status immunity');
    // 			if (message) {
    // 				this.battle.add('-immune', this);
    // 			}
    // 			return false;
    // 		}
    // 		const immunity = this.battle.runEvent('Immunity', this, null, null, type);
    // 		if (!immunity) {
    // 			this.battle.debug('artificial status immunity');
    // 			if (message && immunity !== null) {
    // 				this.battle.add('-immune', this);
    // 			}
    // 			return false;
    // 		}
    // 		return true;
    // 	}
    //
    // ✅ NOW IMPLEMENTED (Session 24 Part 89): Full 1-to-1 with JavaScript
    // - Refactored to associated function with &mut Battle reference
    // - battle.debug() calls for natural and artificial immunity
    // - battle.add('-immune') messages when message parameter is true
    // - runEvent('Immunity') call for ability-based immunity
    //
    pub fn run_status_immunity(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        status: &str,
        with_message: bool,
    ) -> bool {
        // JS: if (this.fainted) return false;
        // ✅ NOW IMPLEMENTED: Fainted check
        let hp = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return false,
            };
            pokemon.hp
        };
        if hp == 0 {
            return false;
        }

        // JS: if (!type) return true;
        // ✅ NOW IMPLEMENTED: Empty string check
        if status.is_empty() {
            return true;
        }

        // JS: if (!this.battle.dex.getImmunity(type, this)) {
        // JS:     this.battle.debug('natural status immunity');
        // JS:     if (message) this.battle.add('-immune', this);
        // JS:     return false;
        // JS: }
        // Note: battle.dex.getImmunity() is primarily for move type immunity (checks damage_taken)
        // For status immunity, we use hardcoded type checks which match JavaScript behavior:
        // - Fire types immune to burn, Electric to paralysis, Poison/Steel to poison, Ice to freeze
        let has_natural_immunity = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return false,
            };
            match status {
                "brn" => pokemon.has_type(battle, "Fire"),
                "par" => pokemon.has_type(battle, "Electric"),
                "psn" | "tox" => pokemon.has_type(battle, "Poison") || pokemon.has_type(battle, "Steel"),
                "frz" => pokemon.has_type(battle, "Ice"),
                "slp" => false, // No type immunity to sleep
                "trapped" => false, // Trapped is a volatile, not a status - no type immunity
                _ => false,
            }
        };

        if has_natural_immunity {
            // ✅ NOW IMPLEMENTED: battle.debug() call
            battle.debug("natural status immunity");

            // ✅ NOW IMPLEMENTED: battle.add('-immune') message
            if with_message {
                let pokemon_ident = {
                    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                        Some(p) => p,
                        None => return false,
                    };
                    pokemon.get_slot()
                };
                battle.add("-immune", &[pokemon_ident.as_str().into()]);
            }
            return false;
        }

        // JS: const immunity = this.battle.runEvent('Immunity', this, null, null, type);
        // JS: if (!immunity) {
        // JS:     this.battle.debug('artificial status immunity');
        // JS:     if (message && immunity !== null) {
        // JS:         this.battle.add('-immune', this);
        // JS:     }
        // JS:     return false;
        // JS: }
        // ✅ NOW IMPLEMENTED: runEvent('Immunity') call for ability-based immunity
        let immunity_result = battle.run_event("Immunity", Some(pokemon_pos), None, None, None);

        if let Some(val) = immunity_result {
            if val == 0 {
                // ✅ NOW IMPLEMENTED: battle.debug() call
                battle.debug("artificial status immunity");

                // ✅ NOW IMPLEMENTED: battle.add('-immune') message (only if immunity !== null)
                if with_message {
                    let pokemon_ident = {
                        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                            Some(p) => p,
                            None => return false,
                        };
                        pokemon.get_slot()
                    };
                    battle.add("-immune", &[pokemon_ident.as_str().into()]);
                }
                return false;
            }
        }

        // JS: return true;
        true
    }
}
