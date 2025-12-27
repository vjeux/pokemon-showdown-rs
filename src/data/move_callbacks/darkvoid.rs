//! Dark Void Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Arg, Battle};
use crate::event::EventResult;

/// onTry(source, target, move) {
///     if (source.species.name === 'Darkrai' || move.hasBounced) {
///         return;
///     }
///     this.add('-fail', source, 'move: Dark Void');
///     this.hint("Only a Pokemon whose form is Darkrai can use this move.");
///     return null;
/// }
pub fn on_try(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Get species name from Dex
    let species_name = battle.dex.species.get(&source.species_id.to_string())
        .map(|s| s.name.as_str())
        .unwrap_or("");

    // Only Darkrai can use this move (or if move has bounced - TODO: need move.hasBounced)
    if species_name == "Darkrai" {
        return EventResult::Continue;
    }

    let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    battle.add("-fail", &[Arg::Pokemon(source), Arg::Str("move: Dark Void")]);
    battle.hint("Only a Pokemon whose form is Darkrai can use this move.");
    EventResult::Null
}

