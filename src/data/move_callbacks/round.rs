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

/// basePowerCallback(target, source, move) {
///     if (move.sourceEffect === 'round') {
///         this.debug('BP doubled');
///         return move.basePower * 2;
///     }
///     return move.basePower;
/// }
pub fn base_power_callback(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onTry(source, target, move) {
///     for (const action of this.queue.list as MoveAction[]) {
///         if (!action.pokemon || !action.move || action.maxMove || action.zmove) continue;
///         if (action.move.id === 'round') {
///             this.queue.prioritizeAction(action, move);
///             return;
///         }
///     }
/// }
pub fn on_try(battle: &mut Battle, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

