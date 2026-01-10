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
    target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    // JavaScript: onHit(target, pokemon) where pokemon.transformInto(target)
    // target_pos = the target of Transform (the Pokemon to copy)
    // source_pos = the Pokemon using Transform (the one who will be transformed)
    let target = target_pos;
    let pokemon = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (!pokemon.transformInto(target))
    //     return false;

    // Call Pokemon::transform_into: pokemon transforms into target
    let success = Pokemon::transform_into(battle, pokemon, target, None);

    if !success {
        return EventResult::NotFail;
    }

    EventResult::Continue
}
