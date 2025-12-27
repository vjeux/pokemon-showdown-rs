//! Hyperspace Fury Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use crate::event::EventResult;
use super::{Status, Effect};

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

