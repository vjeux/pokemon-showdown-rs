//! Last Resort Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onTry(source) {
/// if (source.moveSlots.length < 2) return false; // Last Resort fails unless the user knows at least 2 moves
/// let hasLastResort = false; // User must actually have Last Resort for it to succeed
/// for (const moveSlot of source.moveSlots) {
///     if (moveSlot.id === 'lastresort') {
///         hasLastResort = true;
///         continue;
///     }
///     if (!moveSlot.used) return false;
/// }
/// return hasLastResort;
/// }
pub fn on_try(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

