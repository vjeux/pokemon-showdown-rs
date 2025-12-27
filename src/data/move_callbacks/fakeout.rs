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
    let source = source_pos;

    // if (source.activeMoveActions > 1) {
    let active_move_actions = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.active_move_actions
    };

    if active_move_actions > 1 {
        // this.hint("Fake Out only works on your first turn out.");
        battle.hint("Fake Out only works on your first turn out.");

        // return false;
        return EventResult::Bool(false);
    }

    EventResult::Continue
}

