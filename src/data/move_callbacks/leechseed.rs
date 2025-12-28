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
    use crate::dex_data::ID;

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
        target_pokemon.has_type("grass")
    };

    EventResult::Boolean(!has_grass_type)
}

pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     this.add('-start', target, 'move: Leech Seed');
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.add('-start', target, 'move: Leech Seed');
        let target_arg = {

            let pokemon = match battle.pokemon_at(target.0, target.1) {

                Some(p) => p,

                None => return EventResult::Continue,

            };

            pokemon.get_slot()

        };
        battle.add("-start", &[target_arg, "move: Leech Seed".into()]);

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

        let pokemon = pokemon_pos;

        // const target = this.getAtSlot(pokemon.volatiles['leechseed'].sourceSlot);
        let source_slot = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let leechseed_volatile = match pokemon_pokemon.volatiles.get(&ID::from("leechseed")) {
                Some(v) => v,
                None => return EventResult::Continue,
            };
            leechseed_volatile.source_slot
        };

        let target = match source_slot {
            Some(slot) => {
                // Convert slot index to string and pass to get_at_slot
                let slot_str = slot.to_string();
                battle.get_at_slot(Some(&slot_str))
            },
            None => None,
        };

        // if (!target || target.fainted || target.hp <= 0) {
        //     this.debug('Nothing to leech into');
        //     return;
        // }
        let target_pos = match target {
            Some(pokemon) => (pokemon.side_index, pokemon.position),
            None => {
                battle.debug("Nothing to leech into");
                return EventResult::Continue;
            }
        };

        let target_is_alive = {
            let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => {
                    battle.debug("Nothing to leech into");
                    return EventResult::Continue;
                }
            };
            !target_pokemon.fainted && target_pokemon.hp > 0
        };

        if !target_is_alive {
            battle.debug("Nothing to leech into");
            return EventResult::Continue;
        }

        // const damage = this.damage(pokemon.baseMaxhp / 8, pokemon, target);
        let damage_amount = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.base_maxhp / 8
        };

        let damage = battle.damage(damage_amount, Some(pokemon), Some(target_pos), None, false);

        // if (damage) {
        //     this.heal(damage, target, pokemon);
        // }
        if let Some(damage_amount) = damage {
            if damage_amount > 0 {
                battle.heal(damage_amount, Some(target_pos), Some(pokemon), None);
            }
        }

        EventResult::Continue
    }
}
