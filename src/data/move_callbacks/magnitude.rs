//! Magnitude Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onModifyMove(move, pokemon) {
///     const i = this.random(100);
///     if (i < 5) {
///         move.magnitude = 4;
///         move.basePower = 10;
///     } else if (i < 15) {
///         move.magnitude = 5;
///         move.basePower = 30;
///     } else if (i < 35) {
///         move.magnitude = 6;
///         move.basePower = 50;
///     } else if (i < 65) {
///         move.magnitude = 7;
///         move.basePower = 70;
///     } else if (i < 85) {
///         move.magnitude = 8;
///         move.basePower = 90;
///     } else if (i < 95) {
///         move.magnitude = 9;
///         move.basePower = 110;
///     } else {
///         move.magnitude = 10;
///         move.basePower = 150;
///     }
/// }
pub fn on_modify_move(battle: &mut Battle, move_id: &str, pokemon_pos: (usize, usize)) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onUseMoveMessage(pokemon, target, move) {
///     this.add('-activate', pokemon, 'move: Magnitude', move.magnitude);
/// }
pub fn on_use_move_message(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

