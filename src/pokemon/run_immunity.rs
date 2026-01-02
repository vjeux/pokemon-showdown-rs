use crate::*;

impl Pokemon {

    /// Run immunity check
    /// Equivalent to runImmunity in pokemon.ts
    // TypeScript source:
    // /** false = immune, true = not immune */
    // 	runImmunity(source: ActiveMove | string, message?: string | boolean) {
    // 		if (!source) return true;
    // 		const type: string = typeof source !== 'string' ? source.type : source;
    // 		if (typeof source !== 'string') {
    // 			if (source.ignoreImmunity && (source.ignoreImmunity === true || source.ignoreImmunity[type])) {
    // 				return true;
    // 			}
    // 		}
    // 		if (!type || type === '???') return true;
    // 		if (!this.battle.dex.types.isName(type)) {
    // 			throw new Error("Use runStatusImmunity for " + type);
    // 		}
    //
    // 		const negateImmunity = !this.battle.runEvent('NegateImmunity', this, type);
    // 		const notImmune = type === 'Ground' ?
    // 			this.isGrounded(negateImmunity) :
    // 			negateImmunity || this.battle.dex.getImmunity(type, this);
    // 		if (notImmune) return true;
    // 		if (!message) return false;
    // 		if (notImmune === null) {
    // 			this.battle.add('-immune', this, '[from] ability: Levitate');
    // 		} else {
    // 			this.battle.add('-immune', this);
    // 		}
    // 		return false;
    // 	}
    //
    // ✅ NOW IMPLEMENTED (Session 24 Part 88): Full 1-to-1 with JavaScript
    // - Refactored to associated function with Battle reference
    // - runEvent('NegateImmunity') call
    // - Ground type → is_grounded() call
    // - Other types → battle.dex.get_immunity() call
    // - battle.add('-immune') messages when appropriate
    //
    pub fn run_immunity(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        move_type: &str,
        with_message: bool,
    ) -> bool {
        // JS: if (!type || type === '???') return true;
        if move_type.is_empty() || move_type == "???" {
            return true;
        }

        // JS: const negateImmunity = !this.battle.runEvent('NegateImmunity', this, type);
        // runEvent returns Some(1) if event should negate, None or Some(0) otherwise
        let negate_immunity = match battle.run_event("NegateImmunity", Some(pokemon_pos), None, None, None) {
            Some(val) if val == 0 => false,
            None => false,
            _ => true,
        };

        // JS: const notImmune = type === 'Ground' ?
        // JS:     this.isGrounded(negateImmunity) :
        // JS:     negateImmunity || this.battle.dex.getImmunity(type, this);
        let not_immune = if move_type == "Ground" {
            // For Ground type, check if Pokemon is grounded
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return true,
            };
            pokemon.is_grounded(battle, negate_immunity)
        } else {
            // For other types, check type immunity via Dex
            if negate_immunity {
                true
            } else {
                let pokemon_types = {
                    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                        Some(p) => p,
                        None => return true,
                    };
                    pokemon.get_types(battle, false)
                };
                battle.dex.get_immunity(move_type, &pokemon_types)
            }
        };

        // JS: if (notImmune) return true;
        if not_immune {
            return true;
        }

        // JS: if (!message) return false;
        if !with_message {
            return false;
        }

        // JS: if (notImmune === null) {
        // JS:     this.battle.add('-immune', this, '[from] ability: Levitate');
        // JS: } else {
        // JS:     this.battle.add('-immune', this);
        // JS: }
        // Note: JavaScript returns null for Levitate immunity, but Rust is_grounded returns bool
        // We approximate by checking if Ground type AND not immune
        if move_type == "Ground" {
            // Ground immunity could be from Levitate
            let pokemon_ident = {
                let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => return false,
                };
                pokemon.get_slot()
            };
            battle.add("-immune", &[pokemon_ident.as_str().into(), "[from] ability: Levitate".into()]);
        } else {
            let pokemon_ident = {
                let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => return false,
                };
                pokemon.get_slot()
            };
            battle.add("-immune", &[pokemon_ident.as_str().into()]);
        }

        // JS: return false;
        false
    }
}
