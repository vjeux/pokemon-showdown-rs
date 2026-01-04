//! BattleActions::spreadMoveHit - Spread move hit processing
//!
//! 1:1 port of spreadMoveHit from battle-actions.ts:1043

use crate::*;
use crate::battle::SpreadMoveHitResult;

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
    targets: &[Option<(usize, usize)>],
    source_pos: (usize, usize),
    move_id: &ID,
    hit_effect_id: Option<&ID>,
    is_secondary: bool,
    is_self: bool,
) -> SpreadMoveHitResult {

    // JavaScript initializes damage array to `true` for all targets (line 18-20)
    // JavaScript: for (const i of targets.keys()) { damage[i] = true; }
    // In Rust Option<i32>: Some(1) = true placeholder, Some(0) = 0 damage, None = false/failed
    let mut damages: Vec<Option<i32>> = vec![Some(1); targets.len()];
    let mut final_targets = targets.to_vec();

    // Get move data
    eprintln!("[SPREAD_MOVE_HIT] ENTRY: move={:?}, hit_effect={:?}, is_secondary={}, is_self={}",
        move_id, hit_effect_id, is_secondary, is_self);

    // JavaScript lines 23-25: Get moveData from hitEffect or move
    // JavaScript: let moveData = hitEffect as ActiveMove;
    //             if (!moveData) moveData = move;
    //             if (!moveData.flags) moveData.flags = {};
    let move_data_id = hit_effect_id.unwrap_or(move_id);
    let move_data = match battle.dex.moves().get(move_data_id.as_str()) {
        Some(m) => m.clone(),
        None => return (damages, final_targets),
    };

    // JavaScript lines 26-38: TryHitField/TryHitSide/TryHit events
    // These fire based on move.target to check if field/side/single-target moves can execute
    let target_opt = targets.get(0).copied().flatten();

    if !is_self {
        let target_type = move_data.target.as_str();
        let hit_result = if target_type == "all" {
            // JavaScript: this.battle.singleEvent('TryHitField', moveData, {}, target || null, pokemon, move);
            battle.single_event(
                "TryHitField",
                move_id,
                target_opt,
                Some(source_pos),
                Some(move_id),
            )
        } else if target_type == "foeSide" || target_type == "allySide" || target_type == "allyTeam" {
            // JavaScript: this.battle.singleEvent('TryHitSide', moveData, {}, target || null, pokemon, move);
            battle.single_event(
                "TryHitSide",
                move_id,
                target_opt,
                Some(source_pos),
                Some(move_id),
            )
        } else if target_opt.is_some() {
            // JavaScript: this.battle.singleEvent('TryHit', moveData, {}, target, pokemon, move);
            battle.single_event(
                "TryHit",
                move_id,
                target_opt,
                Some(source_pos),
                Some(move_id),
            )
        } else {
            use crate::event::EventResult;
            EventResult::Number(1) // No event to fire, return true
        };

        // JavaScript lines 33-38: Check hitResult and fail if false
        // if (!hitResult) {
        //     if (hitResult === false) { this.battle.add('-fail', pokemon); this.battle.attrLastMove('[still]'); }
        //     return [[false], targets];
        // }
        // EventResult interpretation:
        //   - Boolean(false) or Number(0) -> hitResult === false, add fail message
        //   - Null -> hitResult === null, return [[false], targets] without message
        //   - Continue, Number(n>0), Boolean(true) -> success, continue
        use crate::event::EventResult;
        let should_fail = match hit_result {
            EventResult::Boolean(false) | EventResult::Number(0) => {
                // JavaScript: if (hitResult === false)
                // this.battle.add('-fail', pokemon);
                // this.battle.attrLastMove('[still]');
                if let Some(source_pokemon) = battle.pokemon_at(source_pos.0, source_pos.1) {
                    let source_ident = format!(
                        "p{}a: {}",
                        source_pos.0 + 1,
                        source_pokemon.set.species
                    );
                    battle.add("-fail", &[crate::battle::Arg::String(source_ident)]);
                    battle.attr_last_move(&["[still]"]);
                }
                true
            }
            EventResult::Null => {
                // JavaScript: hitResult === null, fail without message
                true
            }
            _ => false, // Success, continue
        };

        if should_fail {
            // Return all false damages
            return (vec![None; targets.len()], targets.to_vec());
        }
    }


    // Step 0: check for substitute (JavaScript line 1074-1089)
    // JavaScript: damage = this.tryPrimaryHitEvent(damage, targets, pokemon, move, moveData, isSecondary);
    // tryPrimaryHitEvent fires 'TryPrimaryHit' event which Substitute uses to intercept damage
    if !is_secondary && !is_self {
        // JavaScript: if (move.target !== 'all' && move.target !== 'allyTeam' && move.target !== 'allySide' && move.target !== 'foeSide')
        let target_type = move_data.target.as_str();
        if target_type != "all" && target_type != "allyTeam" && target_type != "allySide" && target_type != "foeSide" {
            // tryPrimaryHitEvent: for (const [i, target] of targets.entries()) {
            //     if (!target) continue;
            //     damage[i] = this.battle.runEvent('TryPrimaryHit', target, pokemon, moveData);
            // }
            for (i, &target_opt) in targets.iter().enumerate() {
                if let Some(target_pos) = target_opt {
                    let result = battle.run_event(
                        "TryPrimaryHit",
                        Some(target_pos),
                        Some(source_pos),
                        Some(move_id),
                        None,
                    );

                    // JavaScript checks: if (damage[i] === this.battle.HIT_SUBSTITUTE) {
                    //     damage[i] = true;
                    //     targets[i] = null;
                    // }
                    // HIT_SUBSTITUTE = 0 in JavaScript, so check for Some(0)
                    if result == Some(0) {
                        damages[i] = Some(1); // true equivalent
                        final_targets[i] = None; // Don't calculate damage for this target
                    }
                }
            }
        }
    }

    // JavaScript lines 53-56: Set damage to true for secondary effects and filter targets
    // for (const i of targets.keys()) {
    //     if (targets[i] && isSecondary && !moveData.self) {
    //         damage[i] = true;
    //     }
    //     if (!damage[i]) targets[i] = false;
    // }
    for i in 0..targets.len() {
        // if (targets[i] && isSecondary && !moveData.self) damage[i] = true;
        if final_targets[i].is_some() && is_secondary && move_data.self_effect.is_none() {
            damages[i] = Some(1); // true
        }
        // if (!damage[i]) targets[i] = false;
        if damages[i].is_none() || damages[i] == Some(0) {
            final_targets[i] = None;
        }
    }


    // Step 1: TryHit event - NOW HANDLED IN try_spread_move_hit BEFORE accuracy check
    // The TryHit event was moved to try_spread_move_hit.rs to run before the accuracy check
    // This ensures that Protect-like moves can block attacks without wasting a PRNG call on accuracy

    // Step 2: Get damage for each target
    // JavaScript: damage = this.getSpreadDamage(damage, targets, pokemon, move, moveData, isSecondary, isSelf);
    // IMPORTANT: Pass final_targets (which has None for misses), not targets
    damages = crate::battle_actions::get_spread_damage(
        battle,
        &damages,
        &final_targets,
        source_pos,
        move_id,
        is_secondary,
        is_self,
    );

    // JavaScript lines 62-63: Filter targets after getSpreadDamage
    // for (const i of targets.keys()) {
    //     if (damage[i] === false) targets[i] = false;
    // }
    for i in 0..damages.len() {
        if damages[i].is_none() {
            final_targets[i] = None;
        }
    }


    // Step 3: Apply damage using spread_damage
    let damage_vals: Vec<Option<i32>> = damages.clone();
    let applied_damages = battle.spread_damage(
        &damage_vals,
        &final_targets,
        Some(source_pos),
        Some(move_id),
        false,
    );

    for (i, &applied) in applied_damages.iter().enumerate() {
        damages[i] = applied;
        if applied.is_none() || applied == Some(0) {
            // Don't clear target on 0 damage - that's still a hit
            // Only clear on None (failed)
            if applied.is_none() {
                final_targets[i] = None;
            }
        }
    }

    // JavaScript lines 68-70: Filter targets after spreadDamage
    // for (const i of targets.keys()) {
    //     if (damage[i] === false) targets[i] = false;
    // }
    for i in 0..damages.len() {
        if damages[i].is_none() {
            final_targets[i] = None;
        }
    }


    // Step 3.5: Trigger move's onHit callback and Hit events for successful hits
    // JavaScript: if (moveData.onHit) { hitResult = this.battle.singleEvent('Hit', moveData, {}, target, source, move); }
    //             this.battle.runEvent('Hit', target, pokemon, move)
    // TODO: JavaScript runs Hit events as part of runMoveEffects (line 73)
    // JavaScript: damage = this.runMoveEffects(damage, targets, pokemon, move, moveData, isSecondary, isSelf);
    // The Hit event is triggered INSIDE runMoveEffects in JavaScript, but Rust does it here separately
    // This may cause timing differences
    for (i, &target) in final_targets.iter().enumerate() {
        if let Some(target_pos) = target {
            // Only trigger Hit if we actually dealt damage or the move succeeded
            if damages[i].is_some() {
                // First trigger the move's onHit callback (if it has one)
                // JavaScript: if (moveData.onHit) { this.battle.singleEvent('Hit', moveData, {}, target, source, move); }
                if !is_self && !is_secondary {
                    battle.single_event(
                        "Hit",
                        move_id,
                        Some(target_pos),
                        Some(source_pos),
                        Some(move_id),
                    );
                }

                // Then trigger the general Hit event
                // JavaScript: if (!isSelf && !isSecondary) { this.battle.runEvent('Hit', target, source, move); }
                if !is_self && !is_secondary {
                    battle.run_event(
                        "Hit",
                        Some(target_pos),
                        Some(source_pos),
                        Some(move_id),
                        None,
                    );
                }
            }
        }
    }

    // Step 3.6: Trigger DamagingHit and AfterHit events
    // JavaScript (battle-actions.ts:1140-1149):
    //   if (damagedDamage.length && !isSecondary && !isSelf) {
    //       this.battle.runEvent('DamagingHit', damagedTargets, pokemon, move, damagedDamage);
    //       if (moveData.onAfterHit) {
    //           for (const t of damagedTargets) {
    //               this.battle.singleEvent('AfterHit', moveData, {}, t, pokemon, move);
    //           }
    //       }
    //   }
    // JavaScript line 105: const pokemonOriginalHP = pokemon.hp;
    // Save source pokemon's original HP before DamagingHit event (for EmergencyExit check)
    let pokemon_original_hp = battle.pokemon_at(source_pos.0, source_pos.1)
        .map(|p| p.hp)
        .unwrap_or(0);

    if !is_secondary && !is_self {
        // Collect targets that actually took damage
        let mut damaged_targets = Vec::new();
        for (i, &target) in final_targets.iter().enumerate() {
            if let Some(target_pos) = target {
                // Check if damage was dealt (not None and not 0)
                if let Some(dmg) = damages[i] {
                    if dmg > 0 {
                        damaged_targets.push(target_pos);
                    }
                }
            }
        }

        if !damaged_targets.is_empty() {
            // Trigger DamagingHit event for all damaged targets
            // Note: JavaScript passes all damaged targets, but we'll trigger individually
            for &target_pos in &damaged_targets {
                battle.run_event(
                    "DamagingHit",
                    Some(target_pos),
                    Some(source_pos),
                    Some(move_id),
                    None,
                );
            }

            // Trigger AfterHit singleEvent for each damaged target
            // JavaScript: this.battle.singleEvent('AfterHit', moveData, {}, t, pokemon, move);
            for &target_pos in &damaged_targets {
                battle.single_event(
                    "AfterHit",
                    move_id,
                    Some(target_pos),
                    Some(source_pos),
                    Some(move_id),
                );
            }

            // JavaScript lines 113-115: EmergencyExit event
            // if (pokemon.hp && pokemon.hp <= pokemon.maxhp / 2 && pokemonOriginalHP > pokemon.maxhp / 2) {
            //     this.battle.runEvent('EmergencyExit', pokemon);
            // }
            if let Some(source_pokemon) = battle.pokemon_at(source_pos.0, source_pos.1) {
                let pokemon_hp = source_pokemon.hp;
                let pokemon_maxhp = source_pokemon.maxhp;

                // Check if source pokemon crossed the 50% HP threshold downward
                if pokemon_hp > 0 && pokemon_hp <= pokemon_maxhp / 2 && pokemon_original_hp > pokemon_maxhp / 2 {
                    battle.run_event("EmergencyExit", Some(source_pos), None, None, None);
                }
            }
        }
    }

    // JS: this.battle.eachEvent('Update'); (line 886 - after hit loop)
    // Call eachEvent("Update") after all hits processed
    if !is_secondary && !is_self {
        battle.each_event("Update", None, None);
    }

    // Step 4: Run move effects (boosts, status, healing, etc.)
    damages = crate::battle_actions::run_move_effects(
        battle,
        &damages,
        &final_targets,
        source_pos,
        &move_data,
        is_secondary,
        is_self,
    );

    eprintln!("[SPREAD_MOVE_HIT T{}] After run_move_effects, damages={:?}", battle.turn, damages);

    // JavaScript (battle-actions.ts:1103-1105):
    //   for (const i of targets.keys()) {
    //     if (!damage[i] && damage[i] !== 0) targets[i] = false;
    //   }
    // Filter targets based on damage array - targets with no damage (and damage !== 0) are filtered out
    let mut filtered_targets = final_targets.clone();
    for (i, damage_opt) in damages.iter().enumerate() {
        // JS: if (!damage[i] && damage[i] !== 0) targets[i] = false;
        // In Rust: if damage is None OR damage is Some(value) where value != 0 is false, filter it out
        // Actually, JS logic is: if damage is falsy (false, null, undefined) AND not exactly 0
        // In Rust: damage_opt is None or Some(0) means keep, otherwise check if Some(non-zero)
        match damage_opt {
            None => {
                // damage[i] is falsy (None/null/undefined), so filter out (set to None)
                filtered_targets[i] = None;
                eprintln!("[SPREAD_MOVE_HIT] Filtering out target {} because damage is None", i);
            }
            Some(0) => {
                // damage[i] === 0, keep the target (JavaScript: !damage && damage !== 0 is false when damage === 0)
                eprintln!("[SPREAD_MOVE_HIT] Keeping target {} because damage is Some(0)", i);
            }
            Some(_) => {
                // damage[i] is a non-zero number, keep the target
                eprintln!("[SPREAD_MOVE_HIT] Keeping target {} because damage is Some(non-zero)", i);
            }
        }
    }

    // Step 4.5: Apply self stat changes (move.self boosts)
    // JavaScript (battle-actions.ts:1116):
    //   if (moveData.self && !move.selfDropped) this.selfDrops(targets, pokemon, move, moveData, isSecondary);
    //
    // TODO: MISSING activeTarget preservation (JavaScript lines 79-88)
    // JavaScript:
    //   const activeTarget = this.battle.activeTarget;
    //   if (moveData.self && !move.selfDropped) this.selfDrops(targets, pokemon, move, moveData, isSecondary);
    //   if (moveData.secondaries) this.secondaries(targets, pokemon, move, moveData, isSelf);
    //   this.battle.activeTarget = activeTarget;
    // Rust needs to preserve and restore activeTarget around self effects and secondaries
    //
    // selfDrops function (battle-actions.ts:1332-1348):
    //   for (const target of targets) {
    //       if (target === false) continue;
    //       if (moveData.self && !move.selfDropped) {
    //           if (!isSecondary && moveData.self.boosts) {
    //               const secondaryRoll = this.battle.random(100);
    //               if (typeof moveData.self.chance === 'undefined' || secondaryRoll < moveData.self.chance) {
    //                   this.moveHit(source, source, move, moveData.self, isSecondary, true);
    //               }
    //               if (!move.multihit) move.selfDropped = true;
    //           } else {
    //               this.moveHit(source, source, move, moveData.self, isSecondary, true);
    //           }
    //       }
    //   }

    // JavaScript lines 79-88: Save and restore activeTarget
    // const activeTarget = this.battle.activeTarget;
    let saved_active_target = battle.active_target;

    eprintln!("[SPREAD_MOVE_HIT] move_data.self_effect = {:?}", move_data.self_effect);
    if let Some(ref self_effect) = move_data.self_effect {
        // JavaScript line 83: Check !move.selfDropped before applying self effects
        // JavaScript: if (moveData.self && !move.selfDropped) this.selfDrops(...)
        let self_dropped = battle.active_move.as_ref().map_or(false, |m| m.self_dropped);

        if !self_dropped {
            eprintln!("[SPREAD_MOVE_HIT] Processing self_effect for move {} (selfDropped={:?})", move_id, self_dropped);
            // JS: for (const target of targets) { if (target === false) continue; ... }
            // We need to loop through targets to match JavaScript behavior
        // (even though we're applying to source, not target)
        // Use filtered_targets to match JavaScript filtering
        for &target in &filtered_targets {
            // JS: if (target === false) continue;
            if target.is_none() {
                eprintln!("[SPREAD_MOVE_HIT] Skipping None target");
                continue;
            }

            eprintln!("[SPREAD_MOVE_HIT] Processing target {:?}, is_secondary={}, has_boosts={}", target, is_secondary, self_effect.boosts.is_some());

            // JS: if (!isSecondary && moveData.self.boosts)
            if !is_secondary && self_effect.boosts.is_some() {
                eprintln!("[SPREAD_MOVE_HIT] Taking boosts branch");
                // JS: const secondaryRoll = this.battle.random(100);
                let secondary_roll = battle.random(100) as i32;

                // JS: if (typeof moveData.self.chance === 'undefined' || secondaryRoll < moveData.self.chance)
                let should_apply = self_effect.chance.map_or(true, |chance| secondary_roll < chance);

                if should_apply {
                    // JS: this.moveHit(source, source, move, moveData.self, isSecondary, true);
                    // Apply ALL self effects (boosts, volatile status, etc.)

                    if let Some(ref boosts) = self_effect.boosts {
                        let boost_array: Vec<(&str, i8)> = boosts
                            .iter()
                            .map(|(stat, &value)| (stat.as_str(), value as i8))
                            .collect();

                        battle.boost(
                            &boost_array,
                            source_pos,
                            Some(source_pos),
                            Some(move_id.as_str()),
                            false,
                            true,
                        );
                    }

                    // Apply volatile status
                    if let Some(ref volatile_status_name) = self_effect.volatile_status_secondary {
                        eprintln!("[SELF_EFFECT] Applying volatile status (boosts branch): {}", volatile_status_name);

                        // JavaScript onStart for lockedmove: this.effectState.trueDuration = this.random(2, 4);
                        // We need to make the PRNG call BEFORE adding the volatile to match JS order
                        if volatile_status_name == "lockedmove" {
                            let duration = battle.random_with_range(2, 4);
                            eprintln!("[SELF_EFFECT] Rolled lockedmove duration: {}", duration);
                        }

                        // Get mutable reference to source pokemon
                        let volatile_id = ID::new(volatile_status_name);
                        Pokemon::add_volatile(battle, source_pos, volatile_id, None, None, None, None);
                        eprintln!("[SELF_EFFECT] Successfully added volatile '{}' to source", volatile_status_name);
                    }
                }

                // JS: if (!move.multihit) move.selfDropped = true;
                // Set selfDropped flag for non-multihit moves
                if let Some(ref mut active_move) = battle.active_move {
                    if active_move.multi_hit.is_none() {
                        active_move.self_dropped = true;
                    }
                }
            } else {
                eprintln!("[SPREAD_MOVE_HIT] Taking else branch (no boosts or is_secondary=true)");
                // JS: this.moveHit(source, source, move, moveData.self, isSecondary, true);
                // When NOT (!isSecondary && has boosts), always apply moveHit (all self effects)

                if let Some(ref boosts) = self_effect.boosts {
                    let boost_array: Vec<(&str, i8)> = boosts
                        .iter()
                        .map(|(stat, &value)| (stat.as_str(), value as i8))
                        .collect();

                    battle.boost(
                        &boost_array,
                        source_pos,
                        Some(source_pos),
                        Some(move_id.as_str()),
                        is_secondary,
                        true,
                    );
                }

                // Apply volatile status (e.g., lockedmove from Outrage)
                if let Some(ref volatile_status_name) = self_effect.volatile_status_secondary {
                    eprintln!("[SELF_EFFECT] Applying volatile status (else branch): {}", volatile_status_name);

                    // JavaScript onStart for lockedmove: this.effectState.trueDuration = this.random(2, 4);
                    // We need to make the PRNG call BEFORE adding the volatile to match JS order
                    if volatile_status_name == "lockedmove" {
                        let duration = battle.random_with_range(2, 4);
                        eprintln!("[SELF_EFFECT] Rolled lockedmove duration: {}", duration);
                    }

                    // Get mutable reference to source pokemon
                    let volatile_id = ID::new(volatile_status_name);
                    Pokemon::add_volatile(battle, source_pos, volatile_id, None, None, None, None);
                    eprintln!("[SELF_EFFECT] Successfully added volatile '{}' to source", volatile_status_name);
                }
            }

            // JS: Only process once per move (not once per target)
            // The JavaScript loop continues through all targets, but the self effect
            // should only apply once. Break after first valid target.
            break;
        }
        } // End of if !self_dropped
    }

    // Step 4.75: Apply move secondaries (boosts, status effects with chance)
    // JavaScript (battle-actions.ts:1120):
    //   if (moveData.secondaries) this.secondaries(targets, pokemon, move, moveData, isSelf);
    //
    // secondaries function (battle-actions.ts:1363-1376):
    //   secondaries(targets, source, move, moveData, isSelf) {
    //     if (!moveData.secondaries) return;
    //     for (const target of targets) {
    //       if (target === false) continue;
    //       const secondaries = this.battle.runEvent("ModifySecondaries", target, source, moveData, moveData.secondaries.slice());
    //       for (const secondary of secondaries) {
    //         const secondaryRoll = this.battle.random(100);
    //         const secondaryOverflow = (secondary.boosts || secondary.self) && this.battle.gen <= 8;
    //         if (typeof secondary.chance === "undefined" || secondaryRoll < (secondaryOverflow ? secondary.chance % 256 : secondary.chance)) {
    //           this.moveHit(target, source, move, secondary, true, isSelf);
    //         }
    //       }
    //     }
    //   }
    // Get secondaries from active_move (which normalizes secondary -> secondaries)
    // JS processes moveData.secondaries (plural) which is normalized from secondary (singular)

    // DEBUG: Check active_move state
    eprintln!("[SPREAD_MOVE_HIT T{}] Before secondaries check: active_move exists={}, move_id={}",
        battle.turn, battle.active_move.is_some(), move_id);
    if let Some(ref active_move) = battle.active_move {
        eprintln!("[SPREAD_MOVE_HIT T{}] active_move.id={}, secondaries.len()={}, secondaries={:?}",
            battle.turn, active_move.id, active_move.secondaries.len(),
            active_move.secondaries.iter().map(|s| format!("{{chance:{:?}, volatileStatus:{:?}}}", s.chance, s.volatile_status)).collect::<Vec<_>>());
    }

    let has_secondaries_to_process = battle.active_move
        .as_ref()
        .map(|m| !m.secondaries.is_empty())
        .unwrap_or(false);

    eprintln!("[SPREAD_MOVE_HIT T{}] Checking secondaries for move {}: has_secondaries={}, filtered_targets.len()={}",
        battle.turn, move_id, has_secondaries_to_process, filtered_targets.len());

    if has_secondaries_to_process {
        // JS: for (const target of targets) { if (target === false) continue; ... }
        // Use filtered_targets instead of final_targets to match JavaScript filtering
        for (i, &target_opt) in filtered_targets.iter().enumerate() {
            if target_opt.is_none() {
                eprintln!("[SPREAD_MOVE_HIT T{}] Skipping None target at index {}", battle.turn, i);
                continue;
            }
            let target_pos = target_opt.unwrap();

            eprintln!("[SPREAD_MOVE_HIT T{}] Processing secondaries for move {} on target {:?}", battle.turn, move_id, target_pos);

            // JS: const secondaries = this.battle.runEvent('ModifySecondaries', target, source, moveData, moveData.secondaries.slice());
            // Call ModifySecondaries event to allow abilities like Shield Dust to filter secondaries
            battle.run_event(
                "ModifySecondaries",
                Some(target_pos),
                Some(source_pos),
                Some(move_id),
                None,
            );

            eprintln!("[SPREAD_MOVE_HIT T{}] After ModifySecondaries event", battle.turn);

            // After ModifySecondaries, check if there are any secondaries left
            // Shield Dust filters out secondaries by modifying active_move.secondaries
            // If all secondaries were filtered out, skip the PRNG call
            let has_secondaries = battle.active_move
                .as_ref()
                .map(|m| !m.secondaries.is_empty())
                .unwrap_or(false);

            if !has_secondaries {
                eprintln!("[SPREAD_MOVE_HIT T{}] No secondaries left after ModifySecondaries (filtered by Shield Dust or similar), skipping PRNG call", battle.turn);
                continue;
            }

            eprintln!("[SPREAD_MOVE_HIT T{}] Secondaries exist, will process {} secondary effects", battle.turn, battle.active_move.as_ref().map(|m| m.secondaries.len()).unwrap_or(0));

            // Get the secondaries array from active_move
            // JS: for (const secondary of secondaries) {
            let secondaries_to_process: Vec<_> = battle.active_move
                .as_ref()
                .map(|m| m.secondaries.clone())
                .unwrap_or_default();

            for secondary_effect in secondaries_to_process {
                eprintln!("[SPREAD_MOVE_HIT T{}] Processing secondary effect, chance={:?}", battle.turn, secondary_effect.chance);
                eprintln!("[SPREAD_MOVE_HIT T{}] Making PRNG call for secondary on target {:?}", battle.turn, target_pos);

                // JS: const secondaryRoll = this.battle.random(100);
                let secondary_roll = battle.random(100) as i32;
                eprintln!("[SPREAD_MOVE_HIT T{}] Secondary roll={}, chance={:?}", battle.turn, secondary_roll, secondary_effect.chance);

                // JS: const secondaryOverflow = (secondary.boosts || secondary.self) && this.battle.gen <= 8;
                // For now, skip the overflow logic (gen 8 and below edge case)
                // JS: if (typeof secondary.chance === "undefined" || secondaryRoll < secondary.chance)
                let should_apply = secondary_effect.chance.map_or(true, |chance| secondary_roll < chance);

                if should_apply {
                // JS: this.moveHit(target, source, move, secondary, true, isSelf);
                // Apply boosts from secondary effect
                if let Some(boosts) = secondary_effect.boosts {
                    let mut boost_array: Vec<(&str, i8)> = Vec::new();
                    if boosts.atk != 0 {
                        boost_array.push(("atk", boosts.atk));
                    }
                    if boosts.def != 0 {
                        boost_array.push(("def", boosts.def));
                    }
                    if boosts.spa != 0 {
                        boost_array.push(("spa", boosts.spa));
                    }
                    if boosts.spd != 0 {
                        boost_array.push(("spd", boosts.spd));
                    }
                    if boosts.spe != 0 {
                        boost_array.push(("spe", boosts.spe));
                    }
                    if boosts.accuracy != 0 {
                        boost_array.push(("accuracy", boosts.accuracy));
                    }
                    if boosts.evasion != 0 {
                        boost_array.push(("evasion", boosts.evasion));
                    }

                    battle.boost(
                        &boost_array,
                        target_pos,          // target
                        Some(source_pos),    // source
                        Some(move_id.as_str()), // effect = move
                        true,                // is_secondary = true
                        is_self,             // is_self
                    );
                }

                // Apply status from secondary effect
                // JS: moveHit applies moveData.status if present
                if let Some(ref status_name) = secondary_effect.status {
                    eprintln!("[SPREAD_MOVE_HIT T{}] Applying status '{}' to {:?}",
                        battle.turn, status_name, target_pos);

                    // Apply status using associated function
                    let status_id = crate::dex_data::ID::new(status_name);
                    let applied = Pokemon::set_status(battle, target_pos, status_id, None, None, false);
                    eprintln!("[SPREAD_MOVE_HIT T{}] Status '{}' applied: {}", battle.turn, status_name, applied);
                }

                // Apply volatile status from secondary effect
                // JS: if (moveData.volatileStatus) {
                //     hitResult = target.addVolatile(moveData.volatileStatus, source, move);
                // }
                if let Some(ref volatile_status_name) = secondary_effect.volatile_status {
                    eprintln!("[SPREAD_MOVE_HIT T{}] Applying volatile status '{}' from secondary to target {:?}",
                        battle.turn, volatile_status_name, target_pos);

                    let volatile_id = crate::dex_data::ID::new(volatile_status_name);
                    Pokemon::add_volatile(battle, target_pos, volatile_id, Some(source_pos), Some(move_id), None, None);
                }

                // Apply side condition from secondary effect
                // JS: if (moveData.sideCondition) {
                //     hitResult = target.side.addSideCondition(moveData.sideCondition, source, move);
                // }
                if let Some(ref side_condition_name) = secondary_effect.side_condition {
                    eprintln!("[SPREAD_MOVE_HIT T{}] Applying side condition '{}' from secondary to side {}",
                        battle.turn, side_condition_name, target_pos.0);

                    let side_condition_id = crate::dex_data::ID::new(side_condition_name);
                    let applied = battle.sides[target_pos.0].add_side_condition(side_condition_id, None);
                    eprintln!("[SPREAD_MOVE_HIT T{}] Side condition '{}' applied: {}", battle.turn, side_condition_name, applied);
                }

                // Apply slot condition from secondary effect
                // JS: if (moveData.slotCondition) {
                //     hitResult = target.side.addSlotCondition(target, moveData.slotCondition, source, move);
                // }
                if let Some(ref slot_condition_name) = secondary_effect.slot_condition {
                    eprintln!("[SPREAD_MOVE_HIT T{}] Applying slot condition '{}' from secondary to {:?}",
                        battle.turn, slot_condition_name, target_pos);

                    let slot_condition_id = crate::dex_data::ID::new(slot_condition_name);
                    let applied = battle.sides[target_pos.0].add_slot_condition(target_pos.1, slot_condition_id, None);
                    eprintln!("[SPREAD_MOVE_HIT T{}] Slot condition '{}' applied: {}", battle.turn, slot_condition_name, applied);
                }

                // Apply pseudo weather from secondary effect
                // JS: if (moveData.pseudoWeather) {
                //     hitResult = this.battle.field.addPseudoWeather(moveData.pseudoWeather, source, move);
                // }
                if let Some(ref pseudo_weather_name) = secondary_effect.pseudo_weather {
                    eprintln!("[SPREAD_MOVE_HIT T{}] Applying pseudo weather '{}' from secondary",
                        battle.turn, pseudo_weather_name);

                    let pseudo_weather_id = crate::dex_data::ID::new(pseudo_weather_name);
                    let applied = battle.field.add_pseudo_weather(pseudo_weather_id, None);
                    eprintln!("[SPREAD_MOVE_HIT T{}] Pseudo weather '{}' applied: {}", battle.turn, pseudo_weather_name, applied);
                }

                // Apply terrain from secondary effect
                // JS: if (moveData.terrain) {
                //     hitResult = this.battle.field.setTerrain(moveData.terrain, source, move);
                // }
                if let Some(ref terrain_name) = secondary_effect.terrain {
                    eprintln!("[SPREAD_MOVE_HIT T{}] Applying terrain '{}' from secondary",
                        battle.turn, terrain_name);

                    let terrain_id = crate::dex_data::ID::new(terrain_name);
                    let applied = battle.field.set_terrain(terrain_id, None);
                    eprintln!("[SPREAD_MOVE_HIT T{}] Terrain '{}' applied: {}", battle.turn, terrain_name, applied);
                }

                // Apply weather from secondary effect
                // JS: if (moveData.weather) {
                //     hitResult = this.battle.field.setWeather(moveData.weather, source, move);
                // }
                if let Some(ref weather_name) = secondary_effect.weather {
                    eprintln!("[SPREAD_MOVE_HIT T{}] Applying weather '{}' from secondary",
                        battle.turn, weather_name);

                    let weather_id = crate::dex_data::ID::new(weather_name);
                    let applied = battle.field.set_weather(weather_id, None);
                    eprintln!("[SPREAD_MOVE_HIT T{}] Weather '{}' applied: {}", battle.turn, weather_name, applied);
                }
            }
            } // End of for secondary_effect in secondaries_to_process
        }
    }

    // JavaScript line 88: Restore activeTarget
    // this.battle.activeTarget = activeTarget;
    battle.active_target = saved_active_target;

    // JavaScript line 91: forceSwitch step (step 6)
    // if (moveData.forceSwitch) damage = this.forceSwitch(damage, targets, pokemon, move);
    // for (const i of targets.keys()) {
    //     if (!damage[i] && damage[i] !== 0) targets[i] = false;
    // }
    if move_data.force_switch {
        // forceSwitch function (battle-actions.ts:1279-1295):
        // for (const [i, target] of targets.entries()) {
        //     if (target && target.hp > 0 && source.hp > 0 && this.battle.canSwitch(target.side)) {
        //         const hitResult = this.battle.runEvent('DragOut', target, source, move);
        //         if (hitResult) {
        //             target.forceSwitchFlag = true;
        //         } else if (hitResult === false && move.category === 'Status') {
        //             this.battle.add('-fail', source);
        //             this.battle.attrLastMove('[still]');
        //             damage[i] = false;
        //         }
        //     }
        // }
        for (i, &target_opt) in filtered_targets.iter().enumerate() {
            if let Some(target_pos) = target_opt {
                // Check conditions: target.hp > 0 && source.hp > 0 && canSwitch
                let (target_hp, source_hp, can_switch) = {
                    let target_hp = battle.pokemon_at(target_pos.0, target_pos.1)
                        .map(|p| p.hp)
                        .unwrap_or(0);
                    let source_hp = battle.pokemon_at(source_pos.0, source_pos.1)
                        .map(|p| p.hp)
                        .unwrap_or(0);
                    let can_switch = battle.can_switch(target_pos.0) > 0;
                    (target_hp, source_hp, can_switch)
                };

                if target_hp > 0 && source_hp > 0 && can_switch {
                    // const hitResult = this.battle.runEvent('DragOut', target, source, move);
                    let hit_result = battle.run_event(
                        "DragOut",
                        Some(target_pos),
                        Some(source_pos),
                        Some(move_id),
                        None,
                    );

                    // if (hitResult) { target.forceSwitchFlag = true; }
                    // run_event returns Option<i32> where Some(0) = false, Some(n>0) = truthy, None = null
                    match hit_result {
                        Some(n) if n > 0 => {
                            // hitResult is truthy
                            if let Some(target_pokemon) = battle.pokemon_at_mut(target_pos.0, target_pos.1) {
                                target_pokemon.force_switch_flag = true;
                            }
                        }
                        Some(0) => {
                            // hitResult === false && move.category === 'Status'
                            if move_data.category == "Status" {
                                // this.battle.add('-fail', source);
                                // this.battle.attrLastMove('[still]');
                                // damage[i] = false;
                                if let Some(source_pokemon) = battle.pokemon_at(source_pos.0, source_pos.1) {
                                    let source_ident = format!(
                                        "p{}a: {}",
                                        source_pos.0 + 1,
                                        source_pokemon.set.species
                                    );
                                    battle.add("-fail", &[crate::battle::Arg::String(source_ident)]);
                                    battle.attr_last_move(&["[still]"]);
                                    damages[i] = None; // false
                                }
                            }
                        }
                        _ => {
                            // hitResult is null or 0 with non-Status move, do nothing
                        }
                    }
                }
            }
        }

        // for (const i of targets.keys()) {
        //     if (!damage[i] && damage[i] !== 0) targets[i] = false;
        // }
        for i in 0..damages.len() {
            if damages[i].is_none() {
                filtered_targets[i] = None;
            }
        }
    }

    // Step 5: Trigger DamagingHit event for abilities that activate on dealing damage
    // JavaScript (battle-actions.ts:961-971):
    //   const damagedTargets = [];
    //   const damagedDamage = [];
    //   for (const [i, t] of targets.entries()) {
    //     if (typeof damage[i] === "number" && t) {
    //       damagedTargets.push(t);
    //       damagedDamage.push(damage[i]);
    //     }
    //   }
    //   const pokemonOriginalHP = pokemon.hp;
    //   if (damagedDamage.length && !isSecondary && !isSelf) {
    //     this.battle.runEvent("DamagingHit", damagedTargets, pokemon, move, damagedDamage);
    //     ...
    //   }
    // Note: DamagingHit event is already called earlier (lines 163-174), no need to call again here
    //
    // TODO: MISSING EmergencyExit event (JavaScript lines 113-115)
    // JavaScript:
    //   if (pokemon.hp && pokemon.hp <= pokemon.maxhp / 2 && pokemonOriginalHP > pokemon.maxhp / 2) {
    //       this.battle.runEvent('EmergencyExit', pokemon);
    //   }
    // Rust doesn't track pokemonOriginalHP or trigger EmergencyExit event


    (damages, final_targets)
}
