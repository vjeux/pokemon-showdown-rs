//! Leech Seed Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryImmunity(target) {
///     return !target.hasType('Grass');
/// }
pub fn on_try_immunity(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // return !target.hasType('Grass');
    let has_grass_type = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.has_type(battle, "grass")
    };

    EventResult::Boolean(!has_grass_type)
}

pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     this.add('-start', target, 'move: Leech Seed');
    /// }
    pub fn on_start(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
        _effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        let target = pokemon_pos;

        // this.add('-start', target, 'move: Leech Seed');
        let target_arg = {
            let pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,

                None => return EventResult::Continue,
            };

            pokemon.get_slot()
        };
        battle.add("-start", &[target_arg.into(), "move: Leech Seed".into()]);

        EventResult::Continue
    }

    /// onResidual(pokemon) {
    ///     const target = this.getAtSlot(pokemon.volatiles['leechseed'].sourceSlot);
    ///     if (!target || target.fainted || target.hp <= 0) {
    ///         this.debug('Nothing to leech into');
    ///         return;
    ///     }
    ///     const damage = this.damage(pokemon.baseMaxhp / 8, pokemon, target);
    ///     if (damage) {
    ///         this.heal(damage, target, pokemon);
    ///     }
    /// }
    pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        eprintln!("[LEECH SEED] on_residual called for pokemon_pos={:?}", pokemon_pos);

        let pokemon = pokemon_pos;

        // const target = this.getAtSlot(pokemon.volatiles['leechseed'].sourceSlot);
        let source_slot = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => {
                    eprintln!("[LEECH SEED] Failed to get pokemon");
                    return EventResult::Continue;
                }
            };
            let leechseed_volatile = match pokemon_pokemon.volatiles.get(&ID::from("leechseed")) {
                Some(v) => v,
                None => {
                    eprintln!("[LEECH SEED] No leechseed volatile found");
                    return EventResult::Continue;
                }
            };
            eprintln!("[LEECH SEED] Found leechseed volatile, source_slot={:?}", leechseed_volatile.source_slot);
            leechseed_volatile.source_slot
        };

        let target = match source_slot {
            Some(slot) => {
                // Convert slot index to string and pass to get_at_slot
                let slot_str = slot.to_string();
                battle.get_at_slot(Some(&slot_str))
            }
            None => None,
        };

        // if (!target || target.fainted || target.hp <= 0) {
        //     this.debug('Nothing to leech into');
        //     return;
        // }
        let target_pos = match target {
            Some(pokemon) => (pokemon.side_index, pokemon.position),
            None => {
                eprintln!("[LEECH SEED] Nothing to leech into");
                return EventResult::Continue;
            }
        };

        let target_is_alive = {
            let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => {
                    eprintln!("[LEECH SEED] Nothing to leech into");
                    return EventResult::Continue;
                }
            };
            !target_pokemon.fainted && target_pokemon.hp > 0
        };

        if !target_is_alive {
            eprintln!("[LEECH SEED] Nothing to leech into");
            return EventResult::Continue;
        }

        // const damage = this.damage(pokemon.baseMaxhp / 8, pokemon, target);
        let damage_amount = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => {
                    eprintln!("[LEECH SEED] Failed to get pokemon for damage calc");
                    return EventResult::Continue;
                }
            };
            pokemon_pokemon.base_maxhp / 8
        };

        eprintln!("[LEECH SEED] Calling damage({}, {:?}, {:?})", damage_amount, pokemon, target_pos);
        let damage = battle.damage(damage_amount, Some(pokemon), Some(target_pos), None, false);
        eprintln!("[LEECH SEED] damage() returned {:?}", damage);

        // if (damage) {
        //     this.heal(damage, target, pokemon);
        // }
        if let Some(damage_amount) = damage {
            if damage_amount > 0 {
                eprintln!("[LEECH SEED] Calling heal({}, {:?}, {:?})", damage_amount, target_pos, pokemon);
                battle.heal(damage_amount, Some(target_pos), Some(pokemon), None);
            } else {
                eprintln!("[LEECH SEED] damage_amount is 0, skipping heal");
            }
        } else {
            eprintln!("[LEECH SEED] damage is None, skipping heal");
        }

        EventResult::Continue
    }
}
