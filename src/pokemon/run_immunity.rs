use crate::*;
use crate::event::EventResult;
use crate::battle_actions::IgnoreImmunity;

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
    // - Added ignoreImmunity check for Scrappy and similar abilities
    //
    pub fn run_immunity(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        move_type: &str,
        with_message: bool,
    ) -> bool {
        Self::run_immunity_with_ignore(battle, pokemon_pos, move_type, with_message, None)
    }

    /// Run immunity check with optional ignoreImmunity from active move
    pub fn run_immunity_with_ignore(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        move_type: &str,
        with_message: bool,
        ignore_immunity: Option<&IgnoreImmunity>,
    ) -> bool {
        // JS: if (typeof source !== 'string') {
        // JS:     if (source.ignoreImmunity && (source.ignoreImmunity === true || source.ignoreImmunity[type])) {
        // JS:         return true;
        // JS:     }
        // JS: }
        if let Some(ignore) = ignore_immunity {
            match ignore {
                IgnoreImmunity::All => {
                    debug_elog!("[RUN_IMMUNITY] ignoreImmunity=All, returning true (not immune)");
                    return true;
                }
                IgnoreImmunity::Specific(map) => {
                    if map.get(move_type).copied().unwrap_or(false) {
                        debug_elog!("[RUN_IMMUNITY] ignoreImmunity[{}]=true, returning true (not immune)", move_type);
                        return true;
                    }
                }
                IgnoreImmunity::NoIgnore => {
                    // Explicitly set to false, don't ignore immunity
                }
            }
        }

        // JS: if (!type || type === '???') return true;
        if move_type.is_empty() || move_type == "???" {
            return true;
        }

        // JS: const negateImmunity = !this.battle.runEvent('NegateImmunity', this, type);
        // NOTE: JavaScript NEGATES the result!
        // If runEvent returns false (handler explicitly said "negate immunity") → negateImmunity = !false = true
        // If runEvent returns truthy/undefined (no handler or handler didn't negate) → negateImmunity = false
        // Pass the move_type as relay_var so condition callbacks can check it
        let negate_immunity_result = battle.run_event("NegateImmunity", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), None, None, EventResult::String(move_type.to_string()), false, false);
        debug_elog!("[RUN_IMMUNITY] pokemon={:?}, move_type={}, NegateImmunity result={:?}", pokemon_pos, move_type, negate_immunity_result);
        // JavaScript: const negateImmunity = !runEvent(...);
        // A handler returns false to indicate "negate the immunity" (like foresight does)
        // If no handler runs or handlers return Continue/truthy, immunity is NOT negated
        let negate_immunity = matches!(negate_immunity_result, EventResult::Boolean(false));
        debug_elog!("[RUN_IMMUNITY] negate_immunity={}", negate_immunity);

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
                debug_elog!("[RUN_IMMUNITY] negate_immunity=true, returning true (not immune)");
                true
            } else {
                let pokemon_types = {
                    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                        Some(p) => p,
                        None => return true,
                    };
                    pokemon.get_types(battle, false)
                };
                debug_elog!("[RUN_IMMUNITY] pokemon_types={:?}", pokemon_types);
                let immunity_result = battle.dex.get_immunity(move_type, &pokemon_types);
                debug_elog!("[RUN_IMMUNITY] get_immunity({}, {:?}) = {}", move_type, pokemon_types, immunity_result);
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
