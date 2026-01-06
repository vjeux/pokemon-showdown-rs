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
        // Get Pokemon's types and check immunity via dex.getImmunity()
        let has_natural_immunity = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return false,
            };
            let pokemon_types = pokemon.get_types(battle, false); // JS: target.getTypes() (excludeAdded defaults to false)
            !battle.dex.get_immunity(status, &pokemon_types)
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
        // Pass status type as EventResult::String so immunity callbacks can check it
        let immunity_result = battle.run_event(
            "Immunity",
            Some(crate::event::EventTarget::Pokemon(pokemon_pos)),
            None,
            None,
            crate::event::EventResult::String(status.to_string()),
            false,
            false,
        );

        // Check if result is falsy (Boolean(false), Number(0), Null)
        if !immunity_result.is_truthy() {
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

        // JS: return true;
        true
    }
}
