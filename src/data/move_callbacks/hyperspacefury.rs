//! Hyperspace Fury Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry(source) {
///     if (source.species.name === 'Hoopa-Unbound') {
///         return;
///     }
///     this.hint("Only a Pokemon whose form is Hoopa Unbound can use this move.");
///     if (source.species.name === 'Hoopa') {
///         this.attrLastMove('[still]');
///         this.add('-fail', source, 'move: Hyperspace Fury', '[forme]');
///         return null;
///     }
///     this.attrLastMove('[still]');
///     this.add('-fail', source, 'move: Hyperspace Fury');
///     return null;
/// }
pub fn on_try(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Only Hoopa-Unbound can use this move
    if source.species == "Hoopa-Unbound" {
        return EventResult::Continue;
    }

    // TODO: battle.hint("Only a Pokemon whose form is Hoopa Unbound can use this move.");

    if source.species == "Hoopa" {
        // TODO: battle.attrLastMove('[still]');
        // TODO: battle.add('-fail', source, 'move: Hyperspace Fury', '[forme]');
        return EventResult::Null;
    }

    // TODO: battle.attrLastMove('[still]');
    // TODO: battle.add('-fail', source, 'move: Hyperspace Fury');
    EventResult::Null
}

