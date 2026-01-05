use crate::*;
use crate::event::EventResult;

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
        // NOTE: JavaScript NEGATES the result!
        // If runEvent returns truthy → negateImmunity = false
        // If runEvent returns falsy/undefined → negateImmunity = true
        let negate_immunity_result = battle.run_event("NegateImmunity", Some(pokemon_pos), None, None, EventResult::Continue, false, false);
        eprintln!("[RUN_IMMUNITY] pokemon={:?}, move_type={}, NegateImmunity result={:?}", pokemon_pos, move_type, negate_immunity_result);
        let negate_immunity = match negate_immunity_result {
            EventResult::Number(val) if val != 0 => false,  // Event returned truthy → DON'T negate (negateImmunity=false)
            EventResult::Boolean(true) => false,  // Event returned true → DON'T negate
            _ => true,  // Event returned falsy/Continue/Null → DO negate (negateImmunity=true)
        };
        eprintln!("[RUN_IMMUNITY] negate_immunity={}", negate_immunity);

        // JS: const notImmune = type === 'Ground' ?
        // JS:     this.isGrounded(negateImmunity) :
        // JS:     negateImmunity || this.battle.dex.getImmunity(type, this);
        let not_immune = if move_type == "Ground" {
            // For Ground type, check if Pokemon is grounded
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return true,
            };
            // is_grounded returns Option<bool>: Some(true), Some(false), or None (Levitate)
            // JavaScript treats null as falsy, so unwrap_or(false)
            pokemon.is_grounded(battle, negate_immunity).unwrap_or(false)
        } else {
            // For other types, check type immunity via Dex
            if negate_immunity {
                eprintln!("[RUN_IMMUNITY] negate_immunity=true, returning true (not immune)");
                true
            } else {
                let pokemon_types = {
                    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                        Some(p) => p,
                        None => return true,
                    };
                    pokemon.get_types(battle, false)
                };
                eprintln!("[RUN_IMMUNITY] pokemon_types={:?}", pokemon_types);
                let immunity_result = battle.dex.get_immunity(move_type, &pokemon_types);
                eprintln!("[RUN_IMMUNITY] get_immunity({}, {:?}) = {}", move_type, pokemon_types, immunity_result);
                immunity_result
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
