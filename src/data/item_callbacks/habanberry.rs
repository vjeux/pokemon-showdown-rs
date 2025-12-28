//! Haban Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSourceModifyDamage(damage, source, target, move) {
///     if (move.type === 'Dragon' && target.getMoveHitData(move).typeMod > 0) {
///         const hitSub = target.volatiles['substitute'] && !move.flags['bypasssub'] && !(move.infiltrates && this.gen >= 6);
///         if (hitSub) return;
/// 
///         if (target.eatItem()) {
///             this.debug('-50% reduction');
///             this.add('-enditem', target, this.effect, '[weaken]');
///             return this.chainModify(0.5);
///         }
///     }
/// }
pub fn on_source_modify_damage(battle: &mut Battle, damage: i32, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onEat() {
///     num: 197,
///     gen: 4,
/// }
pub fn on_eat(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}
