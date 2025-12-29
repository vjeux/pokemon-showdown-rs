//! Scrappy Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyMove(move) {
///     if (!move.ignoreImmunity) move.ignoreImmunity = {};
///     if (move.ignoreImmunity !== true) {
///         move.ignoreImmunity['Fighting'] = true;
///         move.ignoreImmunity['Normal'] = true;
///     }
/// }
pub fn on_modify_move(battle: &mut Battle, move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onTryBoost(boost, target, source, effect) {
///     if (effect.name === 'Intimidate' && boost.atk) {
///         delete boost.atk;
///         this.add('-fail', target, 'unboost', 'Attack', '[from] ability: Scrappy', `[of] ${target}`);
///     }
/// }
pub fn on_try_boost(battle: &mut Battle, boost: &str, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

