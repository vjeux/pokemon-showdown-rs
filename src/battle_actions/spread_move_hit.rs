//! BattleActions::spreadMoveHit - Spread move hit processing
//!
//! 1:1 port of spreadMoveHit from battle-actions.ts:1043

use crate::*;
use crate::battle::SpreadMoveHitResult;

/// Spread move hit - handles individual target hit processing
/// Equivalent to spreadMoveHit() in battle-actions.ts:1043
///
/// Returns (damages, targets) where damages[i] corresponds to targets[i]
pub fn spread_move_hit(
    battle: &mut Battle,
    targets: &[Option<(usize, usize)>],
    source_pos: (usize, usize),
    move_id: &ID,
    is_secondary: bool,
    is_self: bool,
) -> SpreadMoveHitResult {

    let mut damages: Vec<Option<i32>> = vec![Some(0); targets.len()];
    let mut final_targets = targets.to_vec();

    // Get move data
    eprintln!("[SPREAD_MOVE_HIT] ENTRY: move={:?}, is_secondary={}, is_self={}", move_id, is_secondary, is_self);

    let move_data = match battle.dex.moves().get(move_id.as_str()) {
        Some(m) => m.clone(),
        None => return (damages, final_targets),
    };

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

    // Step 1: TryHit event
    if !is_secondary && !is_self {
        for (i, &target) in targets.iter().enumerate() {
            if let Some(target_pos) = target {
                // JavaScript: hitResult = this.battle.singleEvent('TryHit', moveData, {}, target, pokemon, move);
                let hit_result = battle.single_event(
                    "TryHit",
                    move_id,
                    Some(target_pos),
                    Some(source_pos),
                    Some(move_id),
                );

                // If TryHit returns false, the move fails
                if matches!(hit_result, crate::event::EventResult::Boolean(false)) {
                    damages[i] = None;
                    final_targets[i] = None;
                }
            }
        }
    }

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

    // Step 3.5: Trigger Hit events for successful hits
    // JavaScript: this.battle.runEvent('Hit', target, pokemon, move)
    for (i, &target) in final_targets.iter().enumerate() {
        if let Some(target_pos) = target {
            // Only trigger Hit if we actually dealt damage or the move succeeded
            if damages[i].is_some() {
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

    // Step 4.5: Apply self stat changes (move.self boosts)
    // JavaScript (battle-actions.ts:1116):
    //   if (moveData.self && !move.selfDropped) this.selfDrops(targets, pokemon, move, moveData, isSecondary);
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
    eprintln!("[SPREAD_MOVE_HIT] move_data.self_effect = {:?}", move_data.self_effect);
    if let Some(ref self_effect) = move_data.self_effect {
        eprintln!("[SPREAD_MOVE_HIT] Processing self_effect for move {}", move_id);
        // JS: for (const target of targets) { if (target === false) continue; ... }
        // We need to loop through targets to match JavaScript behavior
        // (even though we're applying to source, not target)
        for &target in &final_targets {
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
                        if let Some(side) = battle.sides.get_mut(source_pos.0) {
                            if let Some(pokemon) = side.pokemon.get_mut(source_pos.1) {
                                let volatile_id = crate::dex_data::ID::new(volatile_status_name);
                                pokemon.add_volatile(volatile_id);
                                eprintln!("[SELF_EFFECT] Successfully added volatile '{}' to source {}", volatile_status_name, pokemon.name);
                            }
                        }
                    }
                }

                // JS: if (!move.multihit) move.selfDropped = true;
                // TODO: Set move.selfDropped = true for non-multihit moves
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
                    if let Some(side) = battle.sides.get_mut(source_pos.0) {
                        if let Some(pokemon) = side.pokemon.get_mut(source_pos.1) {
                            let volatile_id = crate::dex_data::ID::new(volatile_status_name);
                            pokemon.add_volatile(volatile_id);
                            eprintln!("[SELF_EFFECT] Successfully added volatile '{}' to source {}", volatile_status_name, pokemon.name);
                        }
                    }
                }
            }

            // JS: Only process once per move (not once per target)
            // The JavaScript loop continues through all targets, but the self effect
            // should only apply once. Break after first valid target.
            break;
        }
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
    if let Some(ref secondary_effect) = move_data.secondary {
        // JS: for (const target of targets) { if (target === false) continue; ... }
        for &target_opt in &final_targets {
            if target_opt.is_none() {
                continue;
            }
            let target_pos = target_opt.unwrap();

            // JS: const secondaryRoll = this.battle.random(100);
            let secondary_roll = battle.random(100) as i32;

            // JS: const secondaryOverflow = (secondary.boosts || secondary.self) && this.battle.gen <= 8;
            // For now, skip the overflow logic (gen 8 and below edge case)
            // JS: if (typeof secondary.chance === "undefined" || secondaryRoll < secondary.chance)
            let should_apply = secondary_effect.chance.map_or(true, |chance| secondary_roll < chance);

            if should_apply {
                // JS: this.moveHit(target, source, move, secondary, true, isSelf);
                // Apply boosts from secondary effect
                if let Some(ref boosts) = secondary_effect.boosts {
                    let boost_array: Vec<(&str, i8)> = boosts
                        .iter()
                        .map(|(stat, &value)| (stat.as_str(), value as i8))
                        .collect();

                    battle.boost(
                        &boost_array,
                        target_pos,          // target
                        Some(source_pos),    // source
                        Some(move_id.as_str()), // effect = move
                        true,                // is_secondary = true
                        is_self,             // is_self
                    );
                }

                // TODO: Handle secondary.status, secondary.volatileStatus, etc.
                // For now, only boosts are implemented
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
    if !is_secondary && !is_self {
        // Collect targets that received damage
        let mut damaged_targets: Vec<(usize, usize)> = Vec::new();

        for (i, &target_opt) in final_targets.iter().enumerate() {
            if let Some(target_pos) = target_opt {
                if let Some(damage_opt) = damages.get(i) {
                    if let Some(damage_val) = damage_opt {
                        if *damage_val != 0 {
                            damaged_targets.push(target_pos);
                        }
                    }
                }
            }
        }

        // Trigger DamagingHit event if any targets took damage
        // JavaScript: this.battle.runEvent("DamagingHit", damagedTargets, pokemon, move, damagedDamage);
        // In JavaScript, runEvent with an array of targets calls the event for each target
        if !damaged_targets.is_empty() {
            for &target_pos in &damaged_targets {
                // The event handler may use random() for abilities like Effect Spore
                battle.run_event(
                    "DamagingHit",
                    Some(target_pos),
                    Some(source_pos),
                    Some(move_id),
                    None,
                );
            }
        }
    }

    (damages, final_targets)
}
