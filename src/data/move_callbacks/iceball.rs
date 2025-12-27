//! Ice Ball Move
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
///     let bp = move.basePower;
///     const iceballData = pokemon.volatiles['iceball'];
///     if (iceballData?.hitCount) {
///         bp *= 2 ** iceballData.contactHitCount;
///     }
///     if (iceballData && pokemon.status !== 'slp') {
///         iceballData.hitCount++;
///         iceballData.contactHitCount++;
///         if (iceballData.hitCount < 5) {
///             iceballData.duration = 2;
///         }
///     }
///     if (pokemon.volatiles['defensecurl']) {
///         bp *= 2;
///     }
///     this.debug(`BP: ${bp}`);
///     return bp;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onModifyMove(move, pokemon, target) {
///     if (pokemon.volatiles['iceball'] || pokemon.status === 'slp' || !target) return;
///     pokemon.addVolatile('iceball');
///     if (move.sourceEffect) pokemon.lastMoveTargetLoc = pokemon.getLocOf(target);
/// }
pub fn on_modify_move(battle: &mut Battle, move_id: &str, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onAfterMove(source, target, move) {
///     const iceballData = source.volatiles["iceball"];
///     if (
///         iceballData &&
///         iceballData.hitCount === 5 &&
///         iceballData.contactHitCount < 5
///         // this conditions can only be met in gen7 and gen8dlc1
///         // see `disguise` and `iceface` abilities in the resp mod folders
///     ) {
///         source.addVolatile("rolloutstorage");
///         source.volatiles["rolloutstorage"].contactHitCount =
///         iceballData.contactHitCount;
///     }
/// }
pub fn on_after_move(battle: &mut Battle, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}


// Condition handlers
pub mod condition {
    use super::*;

    /// onStart() {
    ///     this.effectState.hitCount = 0;
    ///     this.effectState.contactHitCount = 0;
    /// }
    pub fn on_start(battle: &mut Battle) -> MoveHandlerResult {
        // TODO: Implement 1-to-1 from JS
        MoveHandlerResult::Undefined
    }

    /// onResidual(target) {
    ///     if (target.lastMove && target.lastMove.id === 'struggle') {
    ///         // don't lock
    ///         delete target.volatiles['iceball'];
    ///     }
    /// }
    pub fn on_residual(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> MoveHandlerResult {
        // TODO: Implement 1-to-1 from JS
        MoveHandlerResult::Undefined
    }

}
