//! Rollout Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onModifyMove(move, pokemon, target) {
///     if (pokemon.volatiles['rollout'] || pokemon.status === 'slp' || !target) return;
///     pokemon.addVolatile('rollout');
///     if (move.sourceEffect) pokemon.lastMoveTargetLoc = pokemon.getLocOf(target);
/// }
pub fn on_modify_move(battle: &mut Battle, move_id: &str, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onAfterMove(source, target, move) {
///     const rolloutData = source.volatiles["rollout"];
///     if (
///         rolloutData &&
///         rolloutData.hitCount === 5 &&
///         rolloutData.contactHitCount < 5
///         // this conditions can only be met in gen7 and gen8dlc1
///         // see `disguise` and `iceface` abilities in the resp mod folders
///     ) {
///         source.addVolatile("rolloutstorage");
///         source.volatiles["rolloutstorage"].contactHitCount =
///             rolloutData.contactHitCount;
///     }
/// }
pub fn on_after_move(battle: &mut Battle, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}


// Condition handlers
pub mod condition {
    use super::*;

    // TODO: Implement condition handlers
}
