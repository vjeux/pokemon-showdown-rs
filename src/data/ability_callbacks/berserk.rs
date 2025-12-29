//! Berserk Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamage(damage, target, source, effect) {
///     if (
///         effect.effectType === "Move" &&
///         !effect.multihit &&
///         !(effect.hasSheerForce && source.hasAbility('sheerforce'))
///     ) {
///         this.effectState.checkedBerserk = false;
///     } else {
///         this.effectState.checkedBerserk = true;
///     }
/// }
pub fn on_damage(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onTryEatItem(item) {
///     const healingItems = [
///         'aguavberry', 'enigmaberry', 'figyberry', 'iapapaberry', 'magoberry', 'sitrusberry', 'wikiberry', 'oranberry', 'berryjuice',
///     ];
///     if (healingItems.includes(item.id)) {
///         return this.effectState.checkedBerserk;
///     }
///     return true;
/// }
pub fn on_try_eat_item(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onAfterMoveSecondary(target, source, move) {
///     this.effectState.checkedBerserk = true;
///     if (!source || source === target || !target.hp || !move.totalDamage) return;
///     const lastAttackedBy = target.getLastAttackedBy();
///     if (!lastAttackedBy) return;
///     const damage = move.multihit && !move.smartTarget ? move.totalDamage : lastAttackedBy.damage;
///     if (target.hp <= target.maxhp / 2 && target.hp + damage > target.maxhp / 2) {
///         this.boost({ spa: 1 }, target, target);
///     }
/// }
pub fn on_after_move_secondary(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

