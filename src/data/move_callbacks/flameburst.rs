//! Flame Burst Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source, move) {
///     for (const ally of target.adjacentAllies()) {
///         this.damage(ally.baseMaxhp / 16, ally, source, this.dex.conditions.get('Flame Burst'));
///     }
/// }
pub fn on_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onAfterSubDamage(damage, target, source, move) {
///     for (const ally of target.adjacentAllies()) {
///         this.damage(ally.baseMaxhp / 16, ally, source, this.dex.conditions.get('Flame Burst'));
///     }
/// }
pub fn on_after_sub_damage(battle: &mut Battle, damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

