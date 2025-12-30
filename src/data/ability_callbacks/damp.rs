//! Damp Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAnyTryMove(target, source, effect) {
///     if (['explosion', 'mindblown', 'mistyexplosion', 'selfdestruct'].includes(effect.id)) {
///         this.attrLastMove('[still]');
///         this.add('cant', this.effectState.target, 'ability: Damp', effect, `[of] ${target}`);
///         return false;
///     }
/// }
pub fn on_any_try_move(_battle: &mut Battle, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onAnyDamage(damage, target, source, effect) {
///     if (effect && effect.name === 'Aftermath') {
///         return false;
///     }
/// }
pub fn on_any_damage(_battle: &mut Battle, _damage: i32, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    if let Some(effect) = effect_id {
        if effect == "Aftermath" {
            return EventResult::Boolean(false);
        }
    }
    EventResult::Continue
}

