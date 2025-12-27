//! Grass Knot Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target) {
///     const targetWeight = target.getWeight();
///     let bp;
///     if (targetWeight >= 2000) {
///         bp = 120;
///     } else if (targetWeight >= 1000) {
///         bp = 100;
///     } else if (targetWeight >= 500) {
///         bp = 80;
///     } else if (targetWeight >= 250) {
///         bp = 60;
///     } else if (targetWeight >= 100) {
///         bp = 40;
///     } else {
///         bp = 20;
///     }
///     this.debug(`BP: ${bp}`);
///     return bp;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onTryHit(target, source, move) {
///     if (target.volatiles['dynamax']) {
///         this.add('-fail', source, 'move: Grass Knot', '[from] Dynamax');
///         this.attrLastMove('[still]');
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

