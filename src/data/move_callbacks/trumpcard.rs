//! Trump Card Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// basePowerCallback(source, target, move) {
///     const callerMoveId = move.sourceEffect || move.id;
///     const moveSlot = callerMoveId === 'instruct' ? source.getMoveData(move.id) : source.getMoveData(callerMoveId);
///     let bp;
///     if (!moveSlot) {
///         bp = 40;
///     } else {
///         switch (moveSlot.pp) {
///         case 0:
///             bp = 200;
///             break;
///         case 1:
///             bp = 80;
///             break;
///         case 2:
///             bp = 60;
///             break;
///         case 3:
///             bp = 50;
///             break;
///         default:
///             bp = 40;
///             break;
///         }
///     }
/// 
///     this.debug(`BP: ${bp}`);
///     return bp;
/// }
pub fn base_power_callback(battle: &mut Battle, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

