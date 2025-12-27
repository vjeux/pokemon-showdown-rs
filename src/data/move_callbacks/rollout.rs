//! Rollout Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target, move) {
///     let bp = move.basePower;
///     const rolloutData = pokemon.volatiles['rollout'];
///     if (rolloutData?.hitCount) {
///         bp *= 2 ** rolloutData.contactHitCount;
///     }
///     if (rolloutData && pokemon.status !== 'slp') {
///         rolloutData.hitCount++;
///         rolloutData.contactHitCount++;
///         if (rolloutData.hitCount < 5) {
///             rolloutData.duration = 2;
///         }
///     }
///     if (pokemon.volatiles['defensecurl']) {
///         bp *= 2;
///     }
///     this.debug(`BP: ${bp}`);
///     return bp;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onModifyMove(move, pokemon, target) {
///     if (pokemon.volatiles['rollout'] || pokemon.status === 'slp' || !target) return;
///     pokemon.addVolatile('rollout');
///     if (move.sourceEffect) pokemon.lastMoveTargetLoc = pokemon.getLocOf(target);
/// }
pub fn on_modify_move(battle: &mut Battle, move_id: &str, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
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
pub fn on_after_move(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart() {
    ///     this.effectState.hitCount = 0;
    ///     this.effectState.contactHitCount = 0;
    /// }
    pub fn on_start(battle: &mut Battle) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onResidual(target) {
    ///     if (target.lastMove && target.lastMove.id === 'struggle') {
    ///         // don't lock
    ///         delete target.volatiles['rollout'];
    ///     }
    /// }
    pub fn on_residual(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
