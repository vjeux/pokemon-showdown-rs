//! Aroma Veil Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAllyTryAddVolatile(status, target, source, effect) {
///     if (['attract', 'disable', 'encore', 'healblock', 'taunt', 'torment'].includes(status.id)) {
///         if (effect.effectType === 'Move') {
///             const effectHolder = this.effectState.target;
///             this.add('-block', target, 'ability: Aroma Veil', `[of] ${effectHolder}`);
///         }
///         return null;
///     }
/// }
pub fn on_ally_try_add_volatile(battle: &mut Battle, status: Option<&str>, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

