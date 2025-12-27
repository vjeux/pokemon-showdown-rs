//! Wake-Up Slap Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// basePowerCallback(pokemon, target, move) {
///     if (target.status === 'slp' || target.hasAbility('comatose')) {
///         this.debug('BP doubled on sleeping target');
///         return move.basePower * 2;
///     }
///     return move.basePower;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onHit(target) {
///     if (target.status === 'slp') target.cureStatus();
/// }
pub fn on_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

