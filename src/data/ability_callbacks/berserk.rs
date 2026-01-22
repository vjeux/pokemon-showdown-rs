//! Berserk Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
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
pub fn on_damage(battle: &mut Battle, _damage: i32, _target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect: Option<&Effect>) -> EventResult {
    let effect_id = effect.map(|e| e.id.as_str());
    debug_elog!("[BERSERK_ON_DAMAGE] turn={}, effect_id={:?}, target={:?}, battle.effect={:?}",
        battle.turn, effect_id, _target_pos, battle.effect.as_ref().map(|e| (e.id.as_str(), e.effect_type)));
    // if (effect.effectType === "Move" && !effect.multihit && !(effect.hasSheerForce && source.hasAbility('sheerforce'))) {
    let should_check = if let Some(effect_id) = effect_id {
        // Check if effect is a move
        if let Some(move_data) = battle.dex.moves().get(effect_id) {
            // Check if move is not multi-hit
            let is_not_multihit = move_data.multi_hit.is_none();

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

    // this.effectState.checkedBerserk = !should_check;
    // Use with_effect_state to persist in the ability's effect state
    let _result = battle.with_effect_state(|state| {
        state.checked_berserk = Some(!should_check);
        debug_elog!("[BERSERK_ON_DAMAGE] set checked_berserk = {:?}", state.checked_berserk);
    });
    debug_elog!("[BERSERK_ON_DAMAGE] with_effect_state returned: {:?}", _result.is_some());

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
pub fn on_try_eat_item(battle: &mut Battle, item_id: Option<&str>, _pokemon_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    debug_elog!("[BERSERK_ON_TRY_EAT_ITEM] turn={}, item_id={:?}, pokemon={:?}, battle.effect={:?}",
        battle.turn, item_id, _pokemon_pos, battle.effect.as_ref().map(|e| (e.id.as_str(), e.effect_type)));

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

    // if (healingItems.includes(item.id))
    // JavaScript receives item as parameter (passed via relayVar in runEvent)
    if let Some(item_id_str) = item_id {
        let item_id_id = ID::from(item_id_str);
        debug_elog!("[BERSERK_ON_TRY_EAT_ITEM] Checking if item {:?} is in healing_items", item_id_id);
        if healing_items.contains(&item_id_id) {
            debug_elog!("[BERSERK_ON_TRY_EAT_ITEM] Item IS a healing item");
            // return this.effectState.checkedBerserk;
            let checked_berserk = battle.with_effect_state_ref(|state| {
                debug_elog!("[BERSERK_ON_TRY_EAT_ITEM] Reading from state, checked_berserk = {:?}", state.checked_berserk);
                state.checked_berserk
            }).flatten().unwrap_or(true);
            debug_elog!("[BERSERK_ON_TRY_EAT_ITEM] Returning EventResult::Boolean({})", checked_berserk);
            return EventResult::Boolean(checked_berserk);
        }
    }

    // return true;
    EventResult::Boolean(true)
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
pub fn on_after_move_secondary(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {

    // this.effectState.checkedBerserk = true;
    battle.with_effect_state(|state| {
        state.checked_berserk = Some(true);
    });

    // if (!source || source === target || !target.hp || !move.totalDamage) return;
    if source_pos == target_pos {
        return EventResult::Continue;
    }

    let (target_hp, target_maxhp, total_damage, smart_target, multihit) = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        // if (!target.hp) return;
        if target.hp <= 0 {
            return EventResult::Continue;
        }

        let active_move_ref = match active_move {
            Some(am) => am,
            None => return EventResult::Continue,
        };

        // if (!move.totalDamage) return;
        if active_move_ref.total_damage <= 0 {
            return EventResult::Continue;
        }

        let active_move_data = (
            active_move_ref.total_damage,
            active_move_ref.smart_target,
            active_move_ref.multi_hit.is_some() || active_move_ref.multi_hit_type.is_some(),
        );

        (target.hp, target.maxhp, active_move_data.0, active_move_data.1, active_move_data.2)
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

    // const damage = move.multihit && !move.smartTarget ? move.totalDamage : lastAttackedBy.damage;
    let damage = if multihit && !smart_target.unwrap_or(false) {
        total_damage
    } else {
        last_attacked_damage
    };

    // if (target.hp <= target.maxhp / 2 && target.hp + damage > target.maxhp / 2) {
    if target_hp <= target_maxhp / 2 && target_hp + damage > target_maxhp / 2 {
        // this.boost({ spa: 1 }, target, target);
        battle.boost(
            &[("spa", 1)],
            target_pos,
            Some(target_pos),
            Some("berserk"),
            false,
            false,
        );
    }

    EventResult::Continue
}

