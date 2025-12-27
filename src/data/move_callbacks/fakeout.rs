//! Fake Out Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry(source) {
///     if (source.activeMoveActions > 1) {
///         this.hint("Fake Out only works on your first turn out.");
///         return false;
///     }
/// }
pub fn on_try(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    if source.active_move_actions > 1 {
        // TODO: battle.hint("Fake Out only works on your first turn out.");
        return EventResult::Bool(false);
    }

    EventResult::Continue
}

