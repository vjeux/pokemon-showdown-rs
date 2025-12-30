//! Liquid Ooze Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSourceTryHeal(damage, target, source, effect) {
///     this.debug(`Heal is occurring: ${target} <- ${source} :: ${effect.id}`);
///     const canOoze = ['drain', 'leechseed', 'strengthsap'];
///     if (canOoze.includes(effect.id)) {
///         this.damage(damage);
///         return 0;
///     }
/// }
pub fn on_source_try_heal(_battle: &mut Battle, _damage: i32, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

