//! Psycho Shift Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source, move) {
///     if (!source.status) return false;
///     move.status = source.status;
/// }
pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    let source = source_pos;

    // if (!source.status) return false;
    let source_status = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.status.clone()
    };

    if source_status.is_none() {
        return EventResult::Boolean(false);
    }

    // move.status = source.status;
    if let Some(ref mut active_move) = battle.active_move {
        active_move.status = source_status;
    }

    EventResult::Continue
}

