//! Magic Guard Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamage(damage, target, source, effect) {
///     if (effect.effectType !== 'Move') {
///         if (effect.effectType === 'Ability') this.add('-activate', source, 'ability: ' + effect.name);
///         return false;
///     }
/// }
pub fn on_damage(_battle: &mut Battle, _damage: i32, _target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

