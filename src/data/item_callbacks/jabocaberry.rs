//! Jaboca Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onDamagingHit(damage, target, source, move) {
///     if (move.category === 'Physical' && source.hp && source.isActive && !source.hasAbility('magicguard')) {
///         if (target.eatItem()) {
///             this.damage(source.baseMaxhp / (target.hasAbility('ripen') ? 4 : 8), source, target);
///         }
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: (usize, usize), source_pos: (usize, usize)) -> EventResult {
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
        let source_has_magic_guard = source.has_ability(battle, &["magicguard"]);

        (move_is_physical, source_hp, source_is_active, source_has_magic_guard)
    };

    if move_is_physical && source_hp > 0 && source_is_active && !source_has_magic_guard {
        // Phase 1: Check if target has ripen ability
        let target_has_ripen = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target.has_ability(battle, &["ripen"])
        };

        // Phase 2: Eat the item (requires mutable access)
        let ate_item = {
            let target = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            Pokemon::eat_item(battle, target_pos, false, None, None).is_some()
        };

        if !ate_item {
            return EventResult::Continue;
        }

        // Phase 3: Get source's base_maxhp
        let source_base_maxhp = {
            let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source.base_maxhp
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
pub fn on_eat(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // No implementation needed - just metadata
    EventResult::Continue
}
