//! Anger Shell Ability
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
///         this.effectState.checkedAngerShell = false;
///     } else {
///         this.effectState.checkedAngerShell = true;
///     }
/// }
pub fn on_damage(battle: &mut Battle, _damage: i32, _target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    // if (effect.effectType === "Move" && !effect.multihit && !(effect.hasSheerForce && source.hasAbility('sheerforce'))) {
    let should_check = if let Some(effect_id) = effect_id {
        // Check if effect is a move
        if let Some(move_data) = battle.dex.moves().get(effect_id) {
            // Check if move is not multi-hit
            let is_not_multihit = move_data.multihit.is_none();

            // Check if move has Sheer Force and source has sheerforce ability
            let has_sheer_force_boost = if let Some(source_pos) = source_pos {
                let source_has_sheer_force = {
                    let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    source.has_ability(battle, &["sheerforce"])
                };
                move_data.has_sheer_force && source_has_sheer_force
            } else {
                false
            };

            is_not_multihit && !has_sheer_force_boost
        } else {
            false
        }
    } else {
        false
    };

    // this.effectState.checkedAngerShell = !should_check;
    battle.effect_state.checked_anger_shell = Some(!should_check);

    EventResult::Continue
}

/// onTryEatItem(item) {
///     const healingItems = [
///         'aguavberry', 'enigmaberry', 'figyberry', 'iapapaberry', 'magoberry', 'sitrusberry', 'wikiberry', 'oranberry', 'berryjuice',
///     ];
///     if (healingItems.includes(item.id)) {
///         return this.effectState.checkedAngerShell;
///     }
///     return true;
/// }
pub fn on_try_eat_item(battle: &mut Battle, _item_id: Option<&str>, _pokemon_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    // const healingItems = [...]
    let healing_items = vec![
        ID::from("aguavberry"),
        ID::from("enigmaberry"),
        ID::from("figyberry"),
        ID::from("iapapaberry"),
        ID::from("magoberry"),
        ID::from("sitrusberry"),
        ID::from("wikiberry"),
        ID::from("oranberry"),
        ID::from("berryjuice"),
    ];

    // Get the item being eaten from current event
    let item_id = if let Some(ref event) = battle.event {
        event.effect.as_ref().map(|e| e.id.clone())
    } else {
        None
    };

    // if (healingItems.includes(item.id))
    if let Some(item_id) = item_id {
        if healing_items.contains(&item_id) {
            // return this.effectState.checkedAngerShell;
            let checked_anger_shell = battle.effect_state.checked_anger_shell.unwrap_or(true);
            return EventResult::Boolean(checked_anger_shell);
        }
    }

    // return true;
    EventResult::Boolean(true)
}

/// onAfterMoveSecondary(target, source, move) {
///     this.effectState.checkedAngerShell = true;
///     if (!source || source === target || !target.hp || !move.totalDamage) return;
///     const lastAttackedBy = target.getLastAttackedBy();
///     if (!lastAttackedBy) return;
///     const damage = move.multihit ? move.totalDamage : lastAttackedBy.damage;
///     if (target.hp <= target.maxhp / 2 && target.hp + damage > target.maxhp / 2) {
///         this.boost({ atk: 1, spa: 1, spe: 1, def: -1, spd: -1 }, target, target);
///     }
/// }
pub fn on_after_move_secondary(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), _move_id: &str) -> EventResult {
    // this.effectState.checkedAngerShell = true;
    battle.effect_state.checked_anger_shell = Some(true);

    // if (!source || source === target || !target.hp || !move.totalDamage) return;
    if source_pos == target_pos {
        return EventResult::Continue;
    }

    let (target_hp, target_maxhp, total_damage, multihit) = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        // if (!target.hp) return;
        if target.hp <= 0 {
            return EventResult::Continue;
        }

        let active_move_data = if let Some(ref active_move) = battle.active_move {
            // if (!move.totalDamage) return;
            if active_move.total_damage <= 0 {
                return EventResult::Continue;
            }
            (
                active_move.total_damage,
                active_move.multi_hit.is_some() || active_move.multi_hit_type.is_some(),
            )
        } else {
            return EventResult::Continue;
        };

        (target.hp, target.maxhp, active_move_data.0, active_move_data.1)
    };

    // const lastAttackedBy = target.getLastAttackedBy();
    // if (!lastAttackedBy) return;
    let last_attacked_damage = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        match target.get_last_attacked_by() {
            Some(attacker) => attacker.damage,
            None => return EventResult::Continue,
        }
    };

    // const damage = move.multihit ? move.totalDamage : lastAttackedBy.damage;
    let damage = if multihit {
        total_damage
    } else {
        last_attacked_damage
    };

    // if (target.hp <= target.maxhp / 2 && target.hp + damage > target.maxhp / 2) {
    if target_hp <= target_maxhp / 2 && target_hp + damage > target_maxhp / 2 {
        // this.boost({ atk: 1, spa: 1, spe: 1, def: -1, spd: -1 }, target, target);
        battle.boost(
            &[("atk", 1), ("spa", 1), ("spe", 1), ("def", -1), ("spd", -1)],
            target_pos,
            Some(target_pos),
            Some("angershell"),
            false,
            false,
        );
    }

    EventResult::Continue
}

