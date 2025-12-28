//! Stone Axe Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAfterHit(target, source, move) {
///     if (!move.hasSheerForce && source.hp) {
///         for (const side of source.side.foeSidesWithConditions()) {
///             side.addSideCondition('stealthrock');
///         }
///     }
/// }
pub fn on_after_hit(
    _battle: &mut Battle,
    _source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onAfterSubDamage(damage, target, source, move) {
///     if (!move.hasSheerForce && source.hp) {
///         for (const side of source.side.foeSidesWithConditions()) {
///             side.addSideCondition('stealthrock');
///         }
///     }
/// }
pub fn on_after_sub_damage(
    _battle: &mut Battle,
    _damage: i32,
    _target_pos: Option<(usize, usize)>,
    _source_pos: Option<(usize, usize)>,
    _move_id: &str,
) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}
