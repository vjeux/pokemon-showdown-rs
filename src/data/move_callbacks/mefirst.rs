//! Me First Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onTryHit(target, pokemon) {
///     const action = this.queue.willMove(target);
///     if (!action) return false;
///     const move = this.dex.getActiveMove(action.move.id);
///     if (action.zmove || move.isZ || move.isMax) return false;
///     if (target.volatiles['mustrecharge']) return false;
///     if (move.category === 'Status' || move.flags['failmefirst']) return false;
/// 
///     pokemon.addVolatile('mefirst');
///     this.actions.useMove(move, pokemon, { target });
///     return null;
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, pokemon_pos: (usize, usize)) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}


// Condition handlers
pub mod condition {
    use super::*;

    /// onBasePower(basePower) {
    ///     return this.chainModify(1.5);
    /// }
    pub fn on_base_power(battle: &mut Battle, base_power: i32) -> MoveHandlerResult {
        // TODO: Implement 1-to-1 from JS
        MoveHandlerResult::Undefined
    }

}
