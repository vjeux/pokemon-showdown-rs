//! Expanding Force Move
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

/// onBasePower(basePower, source) {
///     if (this.field.isTerrain('psychicterrain') && source.isGrounded()) {
///         this.debug('terrain buff');
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, base_power: i32, source_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onModifyMove(move, source, target) {
///     if (this.field.isTerrain('psychicterrain') && source.isGrounded()) {
///         move.target = 'allAdjacentFoes';
///     }
/// }
pub fn on_modify_move(battle: &mut Battle, move_id: &str, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

