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
pub fn on_ally_try_boost(_battle: &mut Battle, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
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
pub fn on_ally_set_status(_battle: &mut Battle, _status_id: &str, _target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
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
pub fn on_ally_try_add_volatile(_battle: &mut Battle, _status: Option<&str>, _target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

