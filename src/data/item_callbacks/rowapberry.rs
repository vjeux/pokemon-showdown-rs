//! Rowap Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (move.category === 'Special' && source.hp && source.isActive && !source.hasAbility('magicguard')) {
///         if (target.eatItem()) {
///             this.damage(source.baseMaxhp / (target.hasAbility('ripen') ? 4 : 8), source, target);
///         }
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, damage: i32, target_pos: (usize, usize), source_pos: (usize, usize)) -> EventResult {
    // if (move.category === 'Special' && source.hp && source.isActive && !source.hasAbility('magicguard'))
    let (is_special, source_alive_and_active, source_has_magic_guard) = {
        let is_special = match &battle.active_move {
            Some(active_move) => active_move.category == "Special",
            None => return EventResult::Continue,
        };

        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let alive_and_active = source.hp > 0 && source.is_active;
        let has_magic_guard = source.has_ability(&["magicguard"]);

        (is_special, alive_and_active, has_magic_guard)
    };

    if !is_special || !source_alive_and_active || source_has_magic_guard {
        return EventResult::Continue;
    }

    // if (target.eatItem())
    let (item_eaten, target_has_ripen, source_base_maxhp) = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let has_ripen = target.has_ability(&["ripen"]);

        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let base_maxhp = source.base_maxhp;

        let target_mut = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let eaten = target_mut.eat_item(false);

        (eaten.is_some(), has_ripen, base_maxhp)
    };

    if item_eaten {
        // this.damage(source.baseMaxhp / (target.hasAbility('ripen') ? 4 : 8), source, target);
        let divisor = if target_has_ripen { 4 } else { 8 };
        let damage_amount = source_base_maxhp / divisor;

        use crate::dex_data::ID;
        battle.damage(damage_amount, Some(source_pos), Some(target_pos), Some(&ID::from("rowapberry")), false);
    }

    EventResult::Continue
}

/// onEat() {
///     num: 212,
///     gen: 4,
/// }
pub fn on_eat(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // onEat callback has no implementation - just metadata
    EventResult::Continue
}
