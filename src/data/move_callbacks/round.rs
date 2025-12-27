//! Round Move
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
/// for (const action of this.queue.list as MoveAction[]) {
///     if (!action.pokemon || !action.move || action.maxMove || action.zmove) continue;
///     if (action.move.id === 'round') {
///         this.queue.prioritizeAction(action, move);
///         return;
///     }
/// }
/// }
pub fn on_try(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

