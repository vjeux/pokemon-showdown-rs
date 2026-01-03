//! Foresight Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target) {
///     if (target.volatiles['miracleeye']) return false;
/// }
pub fn on_try_hit(
    battle: &mut Battle,
    _source_pos: (usize, usize),
    target_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;

    let target = target_pos;

    // if (target.volatiles['miracleeye']) return false;
    let has_miracle_eye = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.has_volatile(&ID::from("miracleeye"))
    };

    if has_miracle_eye {
        return EventResult::NotFail;
    }

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     this.add('-start', pokemon, 'Foresight');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // this.add('-start', pokemon, 'Foresight');
        let pokemon_arg = {
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_ref.get_slot()
        };

        battle.add(
            "-start",
            &[pokemon_arg.into(), "Foresight".into()],
        );

        EventResult::Continue
    }

    /// onNegateImmunity(pokemon, type) {
    ///     if (pokemon.hasType('Ghost') && ['Normal', 'Fighting'].includes(type)) return false;
    /// }
    pub fn on_negate_immunity(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // Get the type parameter from the event's relay_var_type
        let immunity_type = match &battle.current_event {
            Some(event) => event.relay_var_type.clone(),
            None => return EventResult::Continue,
        };

        // Check if pokemon has the Ghost type
        let has_ghost_type = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.has_type(battle, "Ghost")
        };

        // if (pokemon.hasType('Ghost') && ['Normal', 'Fighting'].includes(type)) return false;
        if let Some(type_str) = immunity_type {
            if has_ghost_type && (type_str == "Normal" || type_str == "Fighting") {
                // return false; - negate immunity, allowing Normal/Fighting to hit Ghost
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
    pub fn on_modify_boost(_battle: &mut Battle) -> EventResult {
        // TODO: This callback needs boosts parameter support in the function signature
        // The TypeScript version receives (boosts) and modifies it in-place
        // For now, implementing a placeholder that returns Continue
        // This needs infrastructure changes to pass mutable boosts reference
        EventResult::Continue
    }
}
