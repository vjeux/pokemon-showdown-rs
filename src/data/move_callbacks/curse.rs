//! Curse Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyMove(move, source, target) {
///     if (!source.hasType('Ghost')) {
///         move.target = move.nonGhostTarget!;
///     } else if (source.isAlly(target)) {
///         move.target = 'randomNormal';
///     }
/// }
pub fn on_modify_move(battle: &mut Battle, move_id: &str, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onTryHit(target, source, move) {
///     if (!source.hasType('Ghost')) {
///         delete move.volatileStatus;
///         delete move.onHit;
///         move.self = { boosts: { spe: -1, atk: 1, def: 1 } };
///     } else if (move.volatileStatus && target.volatiles['curse']) {
///         return false;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onHit(target, source) {
///     this.directDamage(source.maxhp / 2, source, source);
/// }
pub fn on_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(pokemon, source) {
    ///     this.add('-start', pokemon, 'Curse', `[of] ${source}`);
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onResidual(pokemon) {
    ///     this.damage(pokemon.baseMaxhp / 4);
    /// }
    pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
