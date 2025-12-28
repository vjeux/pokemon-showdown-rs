//! Air Balloon Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(target) {
///     if (!target.ignoringItem() && !this.field.getPseudoWeather('gravity')) {
///         this.add('-item', target, 'Air Balloon');
///     }
/// }
pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onDamagingHit(damage, target, source, move) {
///     this.add('-enditem', target, 'Air Balloon');
///     target.item = '';
///     this.clearEffectState(target.itemState);
///     this.runEvent('AfterUseItem', target, null, null, this.dex.items.get('airballoon'));
/// }
pub fn on_damaging_hit(battle: &mut Battle, damage: i32, target_pos: (usize, usize), source_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onAfterSubDamage(damage, target, source, effect) {
///     this.debug('effect: ' + effect.id);
///     if (effect.effectType === 'Move') {
///         this.add('-enditem', target, 'Air Balloon');
///         target.item = '';
///         this.clearEffectState(target.itemState);
///         this.runEvent('AfterUseItem', target, null, null, this.dex.items.get('airballoon'));
///     }
/// }
pub fn on_after_sub_damage(battle: &mut Battle, damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}
