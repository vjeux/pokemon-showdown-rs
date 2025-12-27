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
pub fn on_try(battle: &mut Battle, source_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

