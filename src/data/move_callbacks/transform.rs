//! Transform Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, pokemon) {
///     if (!pokemon.transformInto(target)) {
///         return false;
///     }
/// }
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let pokemon = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (!pokemon.transformInto(target))
    //     return false;

    // The transform_into method needs both &mut self (pokemon) and &Pokemon (target)
    // We need to use unsafe to get both references from battle at once
    let success = {
        // First, verify both pokemon exist
        if battle.pokemon_at(pokemon.0, pokemon.1).is_none() {
            return EventResult::Continue;
        }
        if battle.pokemon_at(target.0, target.1).is_none() {
            return EventResult::Continue;
        }

        // Get both references using unsafe pointer casting
        unsafe {
            let battle_ptr = battle as *mut Battle;
            let target_pokemon = match (*battle_ptr).pokemon_at(target.0, target.1) {
                Some(p) => p as *const _ as *const crate::pokemon::Pokemon,
                None => return EventResult::Continue,
            };
            let pokemon_mut = match (*battle_ptr).pokemon_at_mut(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            pokemon_mut.transform_into(&*target_pokemon)
        }
    };

    if !success {
        return EventResult::NotFail;
    }

    EventResult::Continue
}
