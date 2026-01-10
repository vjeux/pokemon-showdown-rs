//! BattleActions::spreadMoveHit - Spread move hit processing
//!
//! 1:1 port of spreadMoveHit from battle-actions.ts:1043

use crate::*;
use crate::event::EventResult;
use crate::battle::SpreadMoveHitResult;
use crate::battle_actions::{SpreadMoveDamage, DamageResult, SpreadMoveTargets, SpreadMoveTarget, HitEffect, ActiveMove};
use crate::battle::Effect;

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
pub fn spread_move_hit<'a>(
    battle: &mut Battle,
    targets: &SpreadMoveTargets,
    source_pos: (usize, usize),
    active_move: &ActiveMove,
    hit_effect: Option<HitEffect<'a>>,
    is_secondary: bool,
    is_self: bool,
) -> SpreadMoveHitResult {
    let move_id = &active_move.id;
    eprintln!("[SPREAD_MOVE_HIT] ENTRY: move_id={}, targets.len()={}, is_secondary={}, is_self={}",
        move_id, targets.len(), is_secondary, is_self);
    // Initialize damage array with Success (true) for all targets
    // In JS: damage[i] = true for all targets
    let mut damage: SpreadMoveDamage = vec![DamageResult::Success; targets.len()];

    // Clone targets for modification
    // JS: targets is mutable in JavaScript
    let mut targets_mut: SpreadMoveTargets = targets.clone();

    // Get moveData (defaults to active_move if hit_effect is None or is a SecondaryEffect)
    // JS: let moveData = hitEffect as ActiveMove; if (!moveData) moveData = move;
    let move_data: &ActiveMove = match &hit_effect {
        Some(HitEffect::Move(m)) => m,
        _ => active_move,
    };
    let move_data_id = &move_data.id;

    // Get target for TryHit events (first target)
    // JS: const target = targets[0];
    let target_pos = match targets_mut.get(0) {
        Some(SpreadMoveTarget::Target(pos)) => Some(*pos),
        _ => None,
    };

    // Get move target type directly from active_move
    let move_target = &active_move.target;

    // Run TryHitField, TryHitSide, or TryHit events based on move target
    let hit_result = if move_target == "all" && !is_self {
        // JS: hitResult = this.battle.singleEvent('TryHitField', moveData, {}, target || null, pokemon, move);
        battle.single_event(
            "TryHitField",
            &crate::battle::Effect::move_(move_data_id.clone()),
            None,
            target_pos,
            Some(source_pos),
            Some(&Effect::move_(move_id.clone())),
            None,
        )
    } else if (move_target == "foeSide" ||
                move_target == "allySide" ||
                move_target == "allyTeam") && !is_self {
        // JS: hitResult = this.battle.singleEvent('TryHitSide', moveData, {}, target || null, pokemon, move);
        battle.single_event(
            "TryHitSide",
            &crate::battle::Effect::move_(move_data_id.clone()),
            None,
            target_pos,
            Some(source_pos),
            Some(&Effect::move_(move_id.clone())),
            None,
        )
    } else if target_pos.is_some() {
        // JS: hitResult = this.battle.singleEvent('TryHit', moveData, {}, target, pokemon, move);
        battle.single_event(
            "TryHit",
            &crate::battle::Effect::move_(move_data_id.clone()),
            None,
            target_pos,
            Some(source_pos),
            Some(&Effect::move_(move_id.clone())),
            None,
        )
    } else {
        event::EventResult::Boolean(true)
    };

    // Check if hit failed
    // JS: if (!hitResult) { if (hitResult === false) { ... } return [[false], targets]; }
    eprintln!("[SPREAD_MOVE_HIT] After TryHit events: hit_result={:?}", hit_result);
    match hit_result {
        event::EventResult::Boolean(false) => {
            eprintln!("[SPREAD_MOVE_HIT] TryHit returned false, returning Failed");
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
        event::EventResult::NotFail => {
            // JS: NOT_FAIL means move failed but without the fail message
            // The target is removed from processing but no -fail message is added
            eprintln!("[SPREAD_MOVE_HIT] TryHit returned NotFail, returning Failed (silently)");
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
            eprintln!("[SPREAD_MOVE_HIT] Before try_primary_hit_event: damage={:?}", damage);
            damage = crate::battle_actions::try_primary_hit_event(
                battle,
                damage,
                &targets_mut,
                source_pos,
                move_data,
                is_secondary,
            );
            eprintln!("[SPREAD_MOVE_HIT] After try_primary_hit_event: damage={:?}", damage);
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
    eprintln!("[SPREAD_MOVE_HIT] After damage loop: damage={:?}, targets={:?}", damage, targets_mut);

    // 1. call to getDamage
    // JS: damage = this.getSpreadDamage(damage, targets, pokemon, move, moveData, isSecondary, isSelf);
    eprintln!("[SPREAD_MOVE_HIT] About to call get_spread_damage");
    // Get the active move for damage calculation - JavaScript passes ActiveMove directly
    let active_move_for_damage = battle.active_move.clone();
    if let Some(ref active_move) = active_move_for_damage {
        damage = crate::battle_actions::get_spread_damage(
            battle,
            damage,
            &targets_mut,
            source_pos,
            active_move,
            hit_effect.clone(),
            is_secondary,
            is_self,
        );
    } else {
        eprintln!("[SPREAD_MOVE_HIT] WARNING: No active_move available for get_spread_damage");
    }

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
        Some(&crate::battle::Effect::move_(move_id.clone())),
        false, // instafaint
    );

    for i in 0..targets_mut.len() {
        if matches!(damage[i], DamageResult::Failed) {
            targets_mut[i] = SpreadMoveTarget::None;
        }
    }

    // 3. onHit event happens here
    // JS: damage = this.runMoveEffects(damage, targets, pokemon, move, moveData, isSecondary, isSelf);
    // JavaScript: runMoveEffects(damage, targets, source, move: ActiveMove, moveData: ActiveMove, isSecondary?, isSelf?)
    // Both move and moveData are ActiveMove - typically the same unless dealing with secondary effects

    // IMPORTANT: Clone the ActiveMove BEFORE calling run_move_effects!
    // This is critical because run_move_effects may trigger callbacks (like Metronome's onHit)
    // that call use_move for a different move, which would change battle.active_move.
    // JavaScript doesn't have this problem because it passes moveData as a parameter.
    let (has_self_effect, self_dropped, move_data_clone) = {
        if let Some(ref active_move) = battle.active_move {
            let has_self = active_move.self_effect.is_some();
            eprintln!("[SPREAD_MOVE_HIT] BEFORE run_move_effects: move_id={}, has_self_effect={}, self_dropped={}, secondaries.len()={}",
                active_move.id, has_self, active_move.self_dropped, active_move.secondaries.len());
            (has_self, active_move.self_dropped, Some(active_move.clone()))
        } else {
            eprintln!("[SPREAD_MOVE_HIT] No active_move when extracting info BEFORE run_move_effects!");
            (false, false, None)
        }
    };

    // Get active_move pointer (for 'move' parameter)
    let active_move_ptr = {
        match &battle.active_move {
            Some(am) => am as *const _,
            None => panic!("active_move must be set when calling run_move_effects"),
        }
    };

    // For moveData, use hit_effect if provided, otherwise wrap active_move in HitEffect::Move
    // JS: let moveData = hitEffect as ActiveMove; if (!moveData) moveData = move;
    let hit_effect_for_run = match &hit_effect {
        Some(he) => he.clone(),
        None => HitEffect::Move(unsafe { &*active_move_ptr }),
    };

    damage = unsafe {
        crate::battle_actions::run_move_effects(
            battle,
            damage,
            &targets_mut,
            source_pos,
            &*active_move_ptr,
            hit_effect_for_run,
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
    // NOTE: We use the pre-extracted values from BEFORE run_move_effects
    // IMPORTANT: Only process self_drops and secondaries when hit_effect is a Move (not Secondary)
    // JavaScript's moveData is the HitEffect, so for secondaries it's a SecondaryEffect which has no self/secondaries

    // Only call self_drops if:
    // 1. hit_effect is a Move (not a Secondary) - JavaScript checks moveData.self which is undefined for secondaries
    // 2. self_effect exists and selfDropped is false
    let is_hit_effect_move = matches!(hit_effect, Some(HitEffect::Move(_))) || hit_effect.is_none();
    if is_hit_effect_move && has_self_effect && !self_dropped {
        crate::battle_actions::self_drops(
            battle,
            &targets_mut,
            source_pos,
            move_id,
            is_secondary,
        );
    }

    eprintln!("[SPREAD_MOVE_HIT] Reached secondaries section for move_id={}", move_id);

    // 5. secondary effects
    // JS: if (moveData.secondaries) this.secondaries(targets, pokemon, move, moveData, isSelf);
    // Use the cloned move_data from BEFORE run_move_effects
    // IMPORTANT: Only process secondaries when hit_effect is a Move (not Secondary)
    // JavaScript checks moveData.secondaries which is undefined for SecondaryEffect objects
    if is_hit_effect_move {
        if let Some(ref move_data) = move_data_clone {
            if !move_data.secondaries.is_empty() {
                eprintln!("[SPREAD_MOVE_HIT] Calling secondaries for move_id={}", move_id);
                crate::battle_actions::secondaries(
                    battle,
                    &targets_mut,
                    source_pos,
                    move_data,  // move_ (the original move)
                    move_data,  // move_data (the cloned ActiveMove)
                    is_self,
                );
            }
        }
    }

    // Restore activeTarget
    // JS: this.battle.activeTarget = activeTarget;
    battle.active_target = active_target;

    // 6. force switch
    // JS: if (moveData.forceSwitch) damage = this.forceSwitch(damage, targets, pokemon, move);
    let has_force_switch = {
        let move_data = battle.dex.moves.get(move_data_id).expect("Move not found");
        eprintln!("[SPREAD_MOVE_HIT] move={}, force_switch={}", move_data_id, move_data.force_switch);
        move_data.force_switch
    };

    if has_force_switch {
        eprintln!("[SPREAD_MOVE_HIT] Calling force_switch for move={}", move_data_id);
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
            eprintln!("[SPREAD_MOVE_HIT] force_switch completed");
        } else {
            eprintln!("[SPREAD_MOVE_HIT] No active_move ptr, skipping force_switch");
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
    // CRITICAL: Only numeric damage values should be added, not undefined (Status moves)
    let mut damaged_targets: Vec<(usize, usize)> = Vec::new();
    let mut damaged_damage: Vec<i32> = Vec::new();

    for (i, target) in targets_mut.iter().enumerate() {
        // JavaScript: typeof damage[i] === 'number'
        // Only add if damage is a numeric value (not undefined/Success/Failed)
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
    eprintln!("[SPREAD_MOVE_HIT] DamagingHit check: move_id={}, damaged_damage.len()={}, is_secondary={}, is_self={}",
        move_id.as_str(), damaged_damage.len(), is_secondary, is_self);
    if !damaged_damage.is_empty() && !is_secondary && !is_self {
        // JS: this.battle.runEvent('DamagingHit', damagedTargets, pokemon, move, damagedDamage);
        // JavaScript's runEvent loops through each target when given an array
        // In Rust, we need to explicitly loop through each damaged target
        // And pass the corresponding damage amount as the relay variable
        for (i, target_pos) in damaged_targets.iter().enumerate() {
            let damage_amount = damaged_damage[i];
            battle.run_event("DamagingHit", Some(crate::event::EventTarget::Pokemon(*target_pos)), Some(source_pos), Some(&crate::battle::Effect::move_(move_id.clone())), EventResult::Number(damage_amount), false, false);
        }

        // Check if moveData has onAfterHit
        // JS: if (moveData.onAfterHit) { for (const t of damagedTargets) { this.battle.singleEvent('AfterHit', moveData, {}, t, pokemon, move); } }
        if battle.has_move_id_callback(move_data_id, "onAfterHit") {
            for target_pos in &damaged_targets {
                battle.single_event(
                    "AfterHit",
                    &crate::battle::Effect::move_(move_data_id.clone()),
                    None,
                    Some(*target_pos),
                    Some(source_pos),
                    Some(&Effect::move_(move_id.clone())),
                    None,
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
            battle.run_event("EmergencyExit", Some(crate::event::EventTarget::Pokemon(source_pos)), None, None, EventResult::Continue, false, false);
        }
    }

    // JS: return [damage, targets];
    (damage, targets_mut)
}
