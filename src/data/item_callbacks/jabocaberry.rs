//! Jaboca Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (move.category === 'Physical' && source.hp && source.isActive && !source.hasAbility('magicguard')) {
///         if (target.eatItem()) {
///             this.damage(source.baseMaxhp / (target.hasAbility('ripen') ? 4 : 8), source, target);
///         }
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, damage: i32, target_pos: (usize, usize), source_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onEat() {
///     num: 211,
///     gen: 4,
/// }
pub fn on_eat(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}
