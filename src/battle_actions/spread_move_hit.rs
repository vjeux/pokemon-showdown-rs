//! BattleActions::spreadMoveHit - Spread move hit processing
//!
//! 1:1 port of spreadMoveHit from battle-actions.ts:1043

use crate::*;
use crate::battle::SpreadMoveHitResult;
use crate::battle_actions::{SpreadMoveDamage, DamageResult, SpreadMoveTargets, SpreadMoveTarget};

/// Spread move hit - handles individual target hit processing
/// Equivalent to spreadMoveHit() in battle-actions.ts:1043
// spreadMoveHit(
//     targets: SpreadMoveTargets, pokemon: Pokemon, moveOrMoveName: ActiveMove,
//     hitEffect?: Dex.HitEffect, isSecondary?: boolean, isSelf?: boolean
// ): [SpreadMoveDamage, SpreadMoveTargets] {
//     // Hardcoded for single-target purposes
//     // (no spread moves have any kind of onTryHit handler)
//     const target = targets[0];
//     let damage: (number | boolean | undefined)[] = [];
//     for (const i of targets.keys()) {
//         damage[i] = true;
//     }
//     const move = this.dex.getActiveMove(moveOrMoveName);
//     let hitResult: boolean | number | null = true;
//     let moveData = hitEffect as ActiveMove;
//     if (!moveData) moveData = move;
//     if (!moveData.flags) moveData.flags = {};
//     if (move.target === 'all' && !isSelf) {
//         hitResult = this.battle.singleEvent('TryHitField', moveData, {}, target || null, pokemon, move);
//     } else if ((move.target === 'foeSide' || move.target === 'allySide' || move.target === 'allyTeam') && !isSelf) {
//         hitResult = this.battle.singleEvent('TryHitSide', moveData, {}, target || null, pokemon, move);
//     } else if (target) {
//         hitResult = this.battle.singleEvent('TryHit', moveData, {}, target, pokemon, move);
//     }
//     if (!hitResult) {
//         if (hitResult === false) {
//             this.battle.add('-fail', pokemon);
//             this.battle.attrLastMove('[still]');
//         }
//         return [[false], targets]; // single-target only
//     }
//
//     // 0. check for substitute
//     if (!isSecondary && !isSelf) {
//         if (move.target !== 'all' && move.target !== 'allyTeam' && move.target !== 'allySide' && move.target !== 'foeSide') {
//             damage = this.tryPrimaryHitEvent(damage, targets, pokemon, move, moveData, isSecondary);
//         }
//     }
//
//     for (const i of targets.keys()) {
//         if (damage[i] === this.battle.HIT_SUBSTITUTE) {
//             damage[i] = true;
//             targets[i] = null;
//         }
//         if (targets[i] && isSecondary && !moveData.self) {
//             damage[i] = true;
//         }
//         if (!damage[i]) targets[i] = false;
//     }
//     // 1. call to this.battle.getDamage
//     damage = this.getSpreadDamage(damage, targets, pokemon, move, moveData, isSecondary, isSelf);
//
//     for (const i of targets.keys()) {
//         if (damage[i] === false) targets[i] = false;
//     }
//
//     // 2. call to this.battle.spreadDamage
//     damage = this.battle.spreadDamage(damage, targets, pokemon, move);
//
//     for (const i of targets.keys()) {
//         if (damage[i] === false) targets[i] = false;
//     }
//
//     // 3. onHit event happens here
//     damage = this.runMoveEffects(damage, targets, pokemon, move, moveData, isSecondary, isSelf);
//
//     for (const i of targets.keys()) {
//         if (!damage[i] && damage[i] !== 0) targets[i] = false;
//     }
//
//     // steps 4 and 5 can mess with this.battle.activeTarget, which needs to be preserved for Dancer
//     const activeTarget = this.battle.activeTarget;
//
//     // 4. self drops (start checking for targets[i] === false here)
//     if (moveData.self && !move.selfDropped) this.selfDrops(targets, pokemon, move, moveData, isSecondary);
//
//     // 5. secondary effects
//     if (moveData.secondaries) this.secondaries(targets, pokemon, move, moveData, isSelf);
//
//     this.battle.activeTarget = activeTarget;
//
//     // 6. force switch
//     if (moveData.forceSwitch) damage = this.forceSwitch(damage, targets, pokemon, move);
//
//     for (const i of targets.keys()) {
//         if (!damage[i] && damage[i] !== 0) targets[i] = false;
//     }
//
//     const damagedTargets: Pokemon[] = [];
//     const damagedDamage = [];
//     for (const [i, t] of targets.entries()) {
//         if (typeof damage[i] === 'number' && t) {
//             damagedTargets.push(t);
//             damagedDamage.push(damage[i]);
//         }
//     }
//     const pokemonOriginalHP = pokemon.hp;
//     if (damagedDamage.length && !isSecondary && !isSelf) {
//         this.battle.runEvent('DamagingHit', damagedTargets, pokemon, move, damagedDamage);
//         if (moveData.onAfterHit) {
//             for (const t of damagedTargets) {
//                 this.battle.singleEvent('AfterHit', moveData, {}, t, pokemon, move);
//             }
//         }
//         if (pokemon.hp && pokemon.hp <= pokemon.maxhp / 2 && pokemonOriginalHP > pokemon.maxhp / 2) {
//             this.battle.runEvent('EmergencyExit', pokemon);
//         }
//     }
//
//     return [damage, targets];
// }
//
///
/// Returns (damages, targets) where damages[i] corresponds to targets[i]
pub fn spread_move_hit(
    battle: &mut Battle,
    targets: &SpreadMoveTargets,
    source_pos: (usize, usize),
    move_id: &ID,
    hit_effect_id: Option<&ID>,
    is_secondary: bool,
    is_self: bool,
) -> SpreadMoveHitResult {
    // Initialize damage array with Success (true) for all targets
    // In JS: damage[i] = true for all targets
    let mut damage: SpreadMoveDamage = vec![DamageResult::Success; targets.len()];

    // Clone targets for modification
    // JS: targets is mutable in JavaScript
    let mut targets_mut: SpreadMoveTargets = targets.clone();

    // Get moveData ID (defaults to move_id if hit_effect_id is None)
    // JS: let moveData = hitEffect as ActiveMove; if (!moveData) moveData = move;
    let move_data_id = hit_effect_id.unwrap_or(move_id);

    // Get target for TryHit events (first target)
    // JS: const target = targets[0];
    let target_pos = match targets_mut.get(0) {
        Some(SpreadMoveTarget::Target(pos)) => Some(*pos),
        _ => None,
    };

    // Get move target type
    let move_target = {
        let move_data = battle.dex.moves.get(move_id).expect("Move not found");
        move_data.target.clone()
    };

    // Run TryHitField, TryHitSide, or TryHit events based on move target
    let hit_result = if move_target == "all" && !is_self {
        // JS: hitResult = this.battle.singleEvent('TryHitField', moveData, {}, target || null, pokemon, move);
        battle.single_event(
            "TryHitField",
            move_data_id,
            target_pos,
            Some(source_pos),
            Some(move_id),
        )
    } else if (move_target == "foeSide" ||
                move_target == "allySide" ||
                move_target == "allyTeam") && !is_self {
        // JS: hitResult = this.battle.singleEvent('TryHitSide', moveData, {}, target || null, pokemon, move);
        battle.single_event(
            "TryHitSide",
            move_data_id,
            target_pos,
            Some(source_pos),
            Some(move_id),
        )
    } else if target_pos.is_some() {
        // JS: hitResult = this.battle.singleEvent('TryHit', moveData, {}, target, pokemon, move);
        battle.single_event(
            "TryHit",
            move_data_id,
            target_pos,
            Some(source_pos),
            Some(move_id),
        )
    } else {
        event::EventResult::Boolean(true)
    };

    // Check if hit failed
    // JS: if (!hitResult) { if (hitResult === false) { ... } return [[false], targets]; }
    match hit_result {
        event::EventResult::Boolean(false) => {
            let pokemon_ident = {
                let pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
                    Some(p) => p,
                    None => return (vec![DamageResult::Failed], targets_mut),
                };
                format!("p{}a: {}", source_pos.0 + 1, pokemon.set.species)
            };
            battle.add("|-fail|", &[crate::battle::Arg::String(pokemon_ident)]);
            battle.attr_last_move(&["[still]"]);
            return (vec![DamageResult::Failed], targets_mut);
        }
        event::EventResult::Null => {
            return (vec![DamageResult::Failed], targets_mut);
        }
        _ => {}
    }

    // 0. check for substitute
    // JS: if (!isSecondary && !isSelf) { if (move.target !== 'all' && ...) { damage = this.tryPrimaryHitEvent(damage, targets, pokemon, move, moveData, isSecondary); } }
    if !is_secondary && !is_self {
        if move_target != "all" &&
           move_target != "allyTeam" &&
           move_target != "allySide" &&
           move_target != "foeSide" {
            damage = crate::battle_actions::try_primary_hit_event(
                battle,
                damage,
                &targets_mut,
                source_pos,
                move_data_id,
                is_secondary,
            );
        }
    }

    // Handle HIT_SUBSTITUTE and secondary move logic
    // JS: for (const i of targets.keys()) { if (damage[i] === this.battle.HIT_SUBSTITUTE) { damage[i] = true; targets[i] = null; } ... }
    for i in 0..targets_mut.len() {
        // Check for HIT_SUBSTITUTE - indicates substitute blocked the hit
        // JS: if (damage[i] === this.battle.HIT_SUBSTITUTE) { damage[i] = true; targets[i] = null; }
        if matches!(damage[i], DamageResult::HitSubstitute) {
            damage[i] = DamageResult::Success; // Convert to "true"
            targets_mut[i] = SpreadMoveTarget::None; // Remove target from processing
        }

        // JS: if (targets[i] && isSecondary && !moveData.self) { damage[i] = true; }
        if matches!(targets_mut[i], SpreadMoveTarget::Target(_)) && is_secondary {
            let has_self = {
                let move_data = battle.dex.moves.get(move_data_id).expect("Move not found");
                move_data.self_effect.is_some()
            };
            if !has_self {
                damage[i] = DamageResult::Success;
            }
        }

        // JS: if (!damage[i]) targets[i] = false;
        if matches!(damage[i], DamageResult::Failed | DamageResult::Undefined) {
            targets_mut[i] = SpreadMoveTarget::None;
        }
    }

    // 1. call to getDamage
    // JS: damage = this.getSpreadDamage(damage, targets, pokemon, move, moveData, isSecondary, isSelf);
    damage = crate::battle_actions::get_spread_damage(
        battle,
        damage,
        &targets_mut,
        source_pos,
        move_data_id,
        is_secondary,
        is_self,
    );

    // JS: for (const i of targets.keys()) { if (damage[i] === false) targets[i] = false; }
    for i in 0..targets_mut.len() {
        if matches!(damage[i], DamageResult::Failed) {
            targets_mut[i] = SpreadMoveTarget::None;
        }
    }

    // 2. call to spreadDamage
    // JS: damage = this.battle.spreadDamage(damage, targets, pokemon, move);
    damage = battle.spread_damage(
        damage,
        &targets_mut,
        Some(source_pos),
        Some(move_id),
        false, // instafaint
    );

    for i in 0..targets_mut.len() {
        if matches!(damage[i], DamageResult::Failed) {
            targets_mut[i] = SpreadMoveTarget::None;
        }
    }

    // 3. onHit event happens here
    // JS: damage = this.runMoveEffects(damage, targets, pokemon, move, moveData, isSecondary, isSelf);
    // Need to get MoveData separately to avoid borrow issues
    let move_data_ptr = {
        let move_data = battle.dex.moves.get(move_data_id).expect("Move not found");
        move_data as *const _
    };
    damage = unsafe {
        crate::battle_actions::run_move_effects(
            battle,
            damage,
            &targets_mut,
            source_pos,
            &*move_data_ptr,
            is_secondary,
            is_self,
        )
    };

    for i in 0..targets_mut.len() {
        // JS: if (!damage[i] && damage[i] !== 0) targets[i] = false;
        match damage[i] {
            DamageResult::Failed | DamageResult::Undefined => {
                targets_mut[i] = SpreadMoveTarget::None;
            }
            _ => {}
        }
    }

    // Preserve activeTarget for Dancer
    // JS: const activeTarget = this.battle.activeTarget;
    let active_target = battle.active_target;

    // 4. self drops
    // JS: if (moveData.self && !move.selfDropped) this.selfDrops(targets, pokemon, move, moveData, isSecondary);
    let has_self_effect = {
        let move_data_def = battle.dex.moves.get(move_data_id).expect("Move not found");
        move_data_def.self_effect.is_some()
    };

    // TODO: Check move.selfDropped field when it exists in MoveData
    // For now, always call self_drops if self_effect exists
    if has_self_effect {
        crate::battle_actions::self_drops(
            battle,
            &targets_mut,
            source_pos,
            move_id,
            is_secondary,
        );
    }

    // 5. secondary effects
    // JS: if (moveData.secondaries) this.secondaries(targets, pokemon, move, moveData, isSelf);
    let has_secondaries = {
        let move_data = battle.dex.moves.get(move_data_id).expect("Move not found");
        move_data.secondaries.as_ref().map_or(false, |s| !s.is_empty())
    };

    if has_secondaries {
        crate::battle_actions::secondaries(
            battle,
            &targets_mut,
            source_pos,
            move_id,
            is_self,
        );
    }

    // Restore activeTarget
    // JS: this.battle.activeTarget = activeTarget;
    battle.active_target = active_target;

    // 6. force switch
    // JS: if (moveData.forceSwitch) damage = this.forceSwitch(damage, targets, pokemon, move);
    let has_force_switch = {
        let move_data = battle.dex.moves.get(move_data_id).expect("Move not found");
        move_data.force_switch
    };

    if has_force_switch {
        // force_switch needs ActiveMove - get pointer to avoid borrow issues
        let active_move_ptr = battle.active_move.as_ref().map(|am| am as *const _);

        if let Some(ptr) = active_move_ptr {
            damage = unsafe {
                crate::battle_actions::force_switch(
                    battle,
                    damage,
                    &targets_mut,
                    source_pos,
                    &*ptr,
                )
            };
        }
    }

    for i in 0..targets_mut.len() {
        match damage[i] {
            DamageResult::Failed | DamageResult::Undefined => {
                targets_mut[i] = SpreadMoveTarget::None;
            }
            _ => {}
        }
    }

    // Collect damaged targets
    // JS: for (const [i, t] of targets.entries()) { if (typeof damage[i] === 'number' && t) { damagedTargets.push(t); damagedDamage.push(damage[i]); } }
    let mut damaged_targets: Vec<(usize, usize)> = Vec::new();
    let mut damaged_damage: Vec<i32> = Vec::new();

    for (i, target) in targets_mut.iter().enumerate() {
        if let DamageResult::Damage(dmg) = damage[i] {
            if let SpreadMoveTarget::Target(target_pos) = target {
                damaged_targets.push(*target_pos);
                damaged_damage.push(dmg);
            }
        }
    }

    // Get pokemon's original HP before DamagingHit event
    // JS: const pokemonOriginalHP = pokemon.hp;
    let pokemon_original_hp = {
        let pokemon = battle.pokemon_at(source_pos.0, source_pos.1);
        pokemon.map(|p| p.hp).unwrap_or(0)
    };

    // DamagingHit and AfterHit events
    // JS: if (damagedDamage.length && !isSecondary && !isSelf) { ... }
    if !damaged_damage.is_empty() && !is_secondary && !is_self {
        // JS: this.battle.runEvent('DamagingHit', damagedTargets, pokemon, move, damagedDamage);
        battle.run_event(
            "DamagingHit",
            Some(source_pos),
            None,
            Some(move_id),
            None,
        );

        // Check if moveData has onAfterHit
        // JS: if (moveData.onAfterHit) { for (const t of damagedTargets) { this.battle.singleEvent('AfterHit', moveData, {}, t, pokemon, move); } }
        if battle.has_callback(move_data_id, "onAfterHit") {
            for target_pos in &damaged_targets {
                battle.single_event(
                    "AfterHit",
                    move_data_id,
                    Some(*target_pos),
                    Some(source_pos),
                    Some(move_id),
                );
            }
        }

        // Emergency Exit check
        // JS: if (pokemon.hp && pokemon.hp <= pokemon.maxhp / 2 && pokemonOriginalHP > pokemon.maxhp / 2) { this.battle.runEvent('EmergencyExit', pokemon); }
        let (pokemon_hp, pokemon_maxhp) = {
            let pokemon = battle.pokemon_at(source_pos.0, source_pos.1);
            pokemon.map(|p| (p.hp, p.maxhp)).unwrap_or((0, 1))
        };

        if pokemon_hp > 0 && pokemon_hp <= pokemon_maxhp / 2 && pokemon_original_hp > pokemon_maxhp / 2 {
            battle.run_event("EmergencyExit", Some(source_pos), None, None, None);
        }
    }

    // JS: return [damage, targets];
    (damage, targets_mut)
}
