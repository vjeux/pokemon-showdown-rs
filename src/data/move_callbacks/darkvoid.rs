//! Dark Void Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onTry(source, target, move) {
///     if (source.species.name === 'Darkrai' || move.hasBounced) {
///         return;
///     }
///     this.add('-fail', source, 'move: Dark Void');
///     this.hint("Only a Pokemon whose form is Darkrai can use this move.");
///     return null;
/// }
pub fn on_try(battle: &mut Battle, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

