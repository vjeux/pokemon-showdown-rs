//! Flower Veil Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAllyTryBoost(boost, target, source, effect) {
///     if ((source && target === source) || !target.hasType('Grass')) return;
///     let showMsg = false;
///     let i: BoostID;
///     for (i in boost) {
///         if (boost[i]! < 0) {
///             delete boost[i];
///             showMsg = true;
///         }
///     }
///     if (showMsg && !(effect as ActiveMove).secondaries) {
///         const effectHolder = this.effectState.target;
///         this.add('-block', target, 'ability: Flower Veil', `[of] ${effectHolder}`);
///     }
/// }
pub fn on_ally_try_boost(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onAllySetStatus(status, target, source, effect) {
///     if (target.hasType('Grass') && source && target !== source && effect && effect.id !== 'yawn') {
///         this.debug('interrupting setStatus with Flower Veil');
///         if (effect.name === 'Synchronize' || (effect.effectType === 'Move' && !effect.secondaries)) {
///             const effectHolder = this.effectState.target;
///             this.add('-block', target, 'ability: Flower Veil', `[of] ${effectHolder}`);
///         }
///         return null;
///     }
/// }
pub fn on_ally_set_status(battle: &mut Battle, status_id: &str, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onAllyTryAddVolatile(status, target) {
///     if (target.hasType('Grass') && status.id === 'yawn') {
///         this.debug('Flower Veil blocking yawn');
///         const effectHolder = this.effectState.target;
///         this.add('-block', target, 'ability: Flower Veil', `[of] ${effectHolder}`);
///         return null;
///     }
/// }
pub fn on_ally_try_add_volatile(battle: &mut Battle, status: Option<&str>, target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

