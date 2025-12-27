//! Water Sport Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};



// Condition handlers
pub mod condition {
    use super::*;

    /// onFieldStart(field, source) {
    ///     this.add('-fieldstart', 'move: Water Sport', `[of] ${source}`);
    /// }
    pub fn on_field_start(battle: &mut Battle, source_pos: Option<(usize, usize)>) -> MoveHandlerResult {
        // TODO: Implement 1-to-1 from JS
        MoveHandlerResult::Undefined
    }

    /// onBasePower(basePower, attacker, defender, move) {
    ///     if (move.type === 'Fire') {
    ///         this.debug('water sport weaken');
    ///         return this.chainModify([1352, 4096]);
    ///     }
    /// }
    pub fn on_base_power(battle: &mut Battle, base_power: i32, move_id: &str) -> MoveHandlerResult {
        // TODO: Implement 1-to-1 from JS
        MoveHandlerResult::Undefined
    }

    /// onFieldEnd() {
    ///     this.add('-fieldend', 'move: Water Sport');
    /// }
    pub fn on_field_end(battle: &mut Battle) -> MoveHandlerResult {
        // TODO: Implement 1-to-1 from JS
        MoveHandlerResult::Undefined
    }

}
