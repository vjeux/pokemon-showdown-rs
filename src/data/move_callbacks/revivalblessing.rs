//! Revival Blessing Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(source) {
///     if (!source.side.pokemon.filter(ally => ally.fainted).length) {
///         return false;
///     }
/// }
pub fn on_try_hit(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> EventResult {
    let source = source_pos;

    // if (!source.side.pokemon.filter(ally => ally.fainted).length) {
    //     return false;
    // }
    let has_fainted_allies = {
        let side = match battle.sides.get(source.0) {
            Some(s) => s,
            None => return EventResult::Continue,
        };
        side.pokemon.iter().any(|p| p.fainted)
    };

    if !has_fainted_allies {
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}
