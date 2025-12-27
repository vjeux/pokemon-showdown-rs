//! Trump Card Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

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
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

