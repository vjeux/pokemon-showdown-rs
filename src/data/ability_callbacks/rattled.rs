//! Rattled Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (['Dark', 'Bug', 'Ghost'].includes(move.type)) {
///         this.boost({ spe: 1 });
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onAfterBoost(boost, target, source, effect) {
///     if (effect?.name === 'Intimidate' && boost.atk) {
///         this.boost({ spe: 1 });
///     }
/// }
pub fn on_after_boost(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

