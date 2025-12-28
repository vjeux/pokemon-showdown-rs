//! Dream Eater Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryImmunity(target) {
///     return target.status === 'slp' || target.hasAbility('comatose');
/// }
pub fn on_try_immunity(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // return target.status === 'slp' || target.hasAbility('comatose');
    let is_immune = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        // Check if status is slp or has Comatose ability
        target_pokemon.status == Some(ID::from("slp")) || target_pokemon.has_ability(&ID::from("comatose"), battle)
    };

    EventResult::Boolean(is_immune)
}

