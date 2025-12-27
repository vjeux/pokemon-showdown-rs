//! Dark Void Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
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

    // Only Darkrai can use this move (or if move has bounced - TODO: need move.hasBounced)
    if source.species == "Darkrai" {
        return EventResult::Continue;
    }

    // TODO: battle.add('-fail', source, 'move: Dark Void');
    // TODO: battle.hint("Only a Pokemon whose form is Darkrai can use this move.");
    EventResult::Null
}

