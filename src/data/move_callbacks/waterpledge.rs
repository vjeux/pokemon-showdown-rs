//! Water Pledge Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onPrepareHit(target, source, move) {
///     for (const action of this.queue) {
///         if (action.choice !== 'move') continue;
///         const otherMove = action.move;
///         const otherMoveUser = action.pokemon;
///         if (
///             !otherMove || !action.pokemon || !otherMoveUser.isActive ||
///             otherMoveUser.fainted || action.maxMove || action.zmove
///         ) {
///             continue;
///         }
///         if (otherMoveUser.isAlly(source) && ['firepledge', 'grasspledge'].includes(otherMove.id)) {
///             this.queue.prioritizeAction(action, move);
///             this.add('-waiting', source, otherMoveUser);
///             return null;
///         }
///     }
/// }
pub fn on_prepare_hit(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onModifyMove(move) {
///     if (move.sourceEffect === 'grasspledge') {
///         move.type = 'Grass';
///         move.forceSTAB = true;
///         move.sideCondition = 'grasspledge';
///     }
///     if (move.sourceEffect === 'firepledge') {
///         move.type = 'Water';
///         move.forceSTAB = true;
///         move.self = { sideCondition: 'waterpledge' };
///     }
/// }
pub fn on_modify_move(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}


// Condition handlers
pub mod condition {
    use super::*;

    // TODO: Implement condition handlers
}
