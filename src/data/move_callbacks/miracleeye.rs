//! Miracle Eye Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target) {
///     if (target.volatiles['foresight']) return false;
/// }
pub fn on_try_hit(
    battle: &mut Battle,
    _source_pos: (usize, usize),
    target_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;

    let target = target_pos;

    // if (target.volatiles['foresight']) return false;
    let has_foresight = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon
            .volatiles
            .contains_key(&ID::from("foresight"))
    };

    if has_foresight {
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     this.add('-start', pokemon, 'Miracle Eye');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // this.add('-start', pokemon, 'Miracle Eye');
        let pokemon_arg = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.get_slot()
        };

        battle.add("-start", &[pokemon_arg.into(), "Miracle Eye".into()]);

        EventResult::Continue
    }

    /// onNegateImmunity(pokemon, type) {
    ///     if (pokemon.hasType('Dark') && type === 'Psychic') return false;
    /// }
    pub fn on_negate_immunity(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // Get the type parameter from the event's relay_var_type
        let immunity_type = match &battle.current_event {
            Some(event) => event.relay_var_type.clone(),
            None => return EventResult::Continue,
        };

        // Check if pokemon has the Dark type
        let has_dark_type = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.has_type(battle, "Dark")
        };

        // if (pokemon.hasType('Dark') && type === 'Psychic') return false;
        if let Some(type_str) = immunity_type {
            if has_dark_type && type_str == "Psychic" {
                // return false; - negate immunity, allowing Psychic to hit Dark
                return EventResult::Boolean(false);
            }
        }

        EventResult::Continue
    }

    /// onModifyBoost(boosts) {
    ///     if (boosts.evasion && boosts.evasion > 0) {
    ///         boosts.evasion = 0;
    ///     }
    /// }
    pub fn on_modify_boost(battle: &mut Battle) -> EventResult {
        // if (boosts.evasion && boosts.evasion > 0) {
        //     boosts.evasion = 0;
        // }
        if let Some(ref mut event) = battle.current_event {
            if let Some(ref mut boosts) = event.relay_var_boost {
                if boosts.evasion > 0 {
                    boosts.evasion = 0;
                }
            }
        }

        EventResult::Continue
    }
}
