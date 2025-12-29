//! Gulp Missile Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (!source.hp || !source.isActive || target.isSemiInvulnerable()) return;
///     if (['cramorantgulping', 'cramorantgorging'].includes(target.species.id)) {
///         this.damage(source.baseMaxhp / 4, source, target);
///         if (target.species.id === 'cramorantgulping') {
///             this.boost({ def: -1 }, source, target, null, true);
///         } else {
///             source.trySetStatus('par', target, move);
///         }
///         target.formeChange('cramorant', move);
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onSourceTryPrimaryHit(target, source, effect) {
///     if (effect?.id === 'surf' && source.hasAbility('gulpmissile') && source.species.name === 'Cramorant') {
///         const forme = source.hp <= source.maxhp / 2 ? 'cramorantgorging' : 'cramorantgulping';
///         source.formeChange(forme, effect);
///     }
/// }
pub fn on_source_try_primary_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

