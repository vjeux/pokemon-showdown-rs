//! Transform Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

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

    // Call Pokemon::transform_into as an associated function with battle and both positions
    let success = Pokemon::transform_into(battle, pokemon, target, None);

    if !success {
        return EventResult::NotFail;
    }

    EventResult::Continue
}
