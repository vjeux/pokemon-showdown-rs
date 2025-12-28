//! Jaboca Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (move.category === 'Physical' && source.hp && source.isActive && !source.hasAbility('magicguard')) {
///         if (target.eatItem()) {
///             this.damage(source.baseMaxhp / (target.hasAbility('ripen') ? 4 : 8), source, target);
///         }
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, damage: i32, target_pos: (usize, usize), source_pos: (usize, usize)) -> EventResult {
    // if (move.category === 'Physical' && source.hp && source.isActive && !source.hasAbility('magicguard'))
    let (move_is_physical, source_hp, source_is_active, source_has_magic_guard) = {
        let active_move = match &battle.active_move {
            Some(m) => m,
            None => return EventResult::Continue,
        };

        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let move_is_physical = active_move.category == "Physical";
        let source_hp = source.hp;
        let source_is_active = source.is_active;
        let source_has_magic_guard = source.has_ability(&["magicguard"]);

        (move_is_physical, source_hp, source_is_active, source_has_magic_guard)
    };

    if move_is_physical && source_hp > 0 && source_is_active && !source_has_magic_guard {
        // if (target.eatItem())
        let (ate_item, source_base_maxhp, target_has_ripen) = {
            let target = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            let ate_item = target.eat_item(false).is_some();

            if !ate_item {
                return EventResult::Continue;
            }

            // Get target's ripen ability check and source base_maxhp
            let target_has_ripen = target.has_ability(&["ripen"]);

            // Get source's base_maxhp
            let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            let source_base_maxhp = source.base_maxhp;

            (ate_item, source_base_maxhp, target_has_ripen)
        };

        // this.damage(source.baseMaxhp / (target.hasAbility('ripen') ? 4 : 8), source, target);
        let damage_amount = if target_has_ripen {
            source_base_maxhp / 4
        } else {
            source_base_maxhp / 8
        };

        battle.damage(damage_amount, Some(source_pos), Some(target_pos), None, false);
    }

    EventResult::Continue
}

/// onEat() {
///     num: 211,
///     gen: 4,
/// }
pub fn on_eat(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // No implementation needed - just metadata
    EventResult::Continue
}
