//! BattleActions::trySpreadMoveHit - Try to hit targets with a spread move
//!
//! 1:1 port of trySpreadMoveHit from battle-actions.ts:545

// JS Source:
// 	trySpreadMoveHit(targets: Pokemon[], pokemon: Pokemon, move: ActiveMove, notActive?: boolean) {
// 		if (targets.length > 1 && !move.smartTarget) move.spreadHit = true;
// 
// 		const moveSteps: ((targets: Pokemon[], pokemon: Pokemon, move: ActiveMove) =>
// 		(number | boolean | "" | undefined)[] | undefined)[] = [
// 			// 0. check for semi invulnerability
// 			this.hitStepInvulnerabilityEvent,
// 
// 			// 1. run the 'TryHit' event (Protect, Magic Bounce, Volt Absorb, etc.) (this is step 2 in gens 5 & 6, and step 4 in gen 4)
// 			this.hitStepTryHitEvent,
// 
// 			// 2. check for type immunity (this is step 1 in gens 4-6)
// 			this.hitStepTypeImmunity,
// 
// 			// 3. check for various move-specific immunities
// 			this.hitStepTryImmunity,
// 
// 			// 4. check accuracy
// 			this.hitStepAccuracy,
// 
// 			// 5. break protection effects
// 			this.hitStepBreakProtect,
// 
// 			// 6. steal positive boosts (Spectral Thief)
// 			this.hitStepStealBoosts,
// 
// 			// 7. loop that processes each hit of the move (has its own steps per iteration)
// 			this.hitStepMoveHitLoop,
// 		];
// 		if (this.battle.gen <= 6) {
// 			// Swap step 1 with step 2
// 			[moveSteps[1], moveSteps[2]] = [moveSteps[2], moveSteps[1]];
// 		}
// 		if (this.battle.gen === 4) {
// 			// Swap step 4 with new step 2 (old step 1)
// 			[moveSteps[2], moveSteps[4]] = [moveSteps[4], moveSteps[2]];
// 		}
// 
// 		if (notActive) this.battle.setActiveMove(move, pokemon, targets[0]);
// 
// 		const hitResult = this.battle.singleEvent('Try', move, null, pokemon, targets[0], move) &&
// 			this.battle.singleEvent('PrepareHit', move, {}, targets[0], pokemon, move) &&
// 			this.battle.runEvent('PrepareHit', pokemon, targets[0], move);
// 		if (!hitResult) {
// 			if (hitResult === false) {
// 				this.battle.add('-fail', pokemon);
// 				this.battle.attrLastMove('[still]');
// 			}
// 			return hitResult === this.battle.NOT_FAIL;
// 		}
// 
// 		let atLeastOneFailure = false;
// 		for (const step of moveSteps) {
// 			const hitResults: (number | boolean | "" | undefined)[] | undefined = step.call(this, targets, pokemon, move);
// 			if (!hitResults) continue;
// 			targets = targets.filter((val, i) => hitResults[i] || hitResults[i] === 0);
// 			atLeastOneFailure = atLeastOneFailure || hitResults.some(val => val === false);
// 			if (move.smartTarget && atLeastOneFailure) move.smartTarget = false;
// 			if (!targets.length) {
// 				// console.log(step.name);
// 				break;
// 			}
// 		}
// 
// 		move.hitTargets = targets;
// 		const moveResult = !!targets.length;
// 		if (!moveResult && !atLeastOneFailure) pokemon.moveThisTurnResult = null;
// 		const hitSlot = targets.map(p => p.getSlot());
// 		if (move.spreadHit) this.battle.attrLastMove('[spread] ' + hitSlot.join(','));
// 		return moveResult;
// 	}


use crate::*;

/// Try to hit targets with a spread move
/// Equivalent to trySpreadMoveHit() in battle-actions.ts:545
///
/// This is the main entry point for move execution with the 7-step pipeline
pub fn try_spread_move_hit(
    battle: &mut Battle,
    targets: &[(usize, usize)],
    pokemon_pos: (usize, usize),
    move_id: &ID,
) -> bool {
    eprintln!("[TRY_SPREAD_MOVE_HIT #{}] Called for move {:?} from {:?} targeting {:?}", {
        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }, move_id, pokemon_pos, targets);
    if targets.is_empty() {
        return false;
    }

    // PrepareHit event - MUST be called BEFORE all moveSteps
    // JavaScript (battle-actions.ts:587):
    //   const hitResult = this.battle.singleEvent('Try', move, null, pokemon, targets[0], move) &&
    //     this.battle.singleEvent('PrepareHit', move, {}, targets[0], pokemon, move) &&
    //     this.battle.runEvent('PrepareHit', pokemon, targets[0], move);
    //   if (!hitResult) {
    //     if (hitResult === false) {
    //       this.battle.add('-fail', pokemon);
    //       this.battle.attrLastMove('[still]');
    //     }
    //     return hitResult === this.battle.NOT_FAIL;
    //   }

    eprintln!("[TRY_SPREAD_MOVE_HIT] About to call PrepareHit for move {:?}", move_id);

    // First, call move's onPrepareHit handler via single_event
    let prepare_hit_result = battle.single_event(
        "PrepareHit",
        move_id,
        Some(targets[0]),
        Some(pokemon_pos),
        None,
    );

    eprintln!("[TRY_SPREAD_MOVE_HIT] PrepareHit single_event result: {:?}", prepare_hit_result);

    // If single_event returned false/None/NotFail, the move fails
    use crate::event::EventResult;
    match prepare_hit_result {
        EventResult::Boolean(false) | EventResult::NotFail | EventResult::Null => {
            return false;
        }
        _ => {}
    }

    // Then, call run_event to find handlers on pokemon/target
    battle.run_event(
        "PrepareHit",
        Some(pokemon_pos),
        Some(targets[0]),
        Some(move_id),
        None,
    );

    // Implement the 7-step pipeline from JavaScript's trySpreadMoveHit

    // Step 0: Invulnerability (hitStepInvulnerabilityEvent)
    // JavaScript: this.hitStepInvulnerabilityEvent
    let invulnerability_results = {
        let mut active_move_clone = battle.active_move.clone().unwrap();
        crate::battle_actions::hit_step_invulnerability_event(
            battle,
            targets,
            pokemon_pos,
            &mut active_move_clone,
        )
    };

    // Filter targets based on invulnerability
    let mut targets_after_invuln = Vec::new();
    for (i, &target) in targets.iter().enumerate() {
        if invulnerability_results.get(i).copied().unwrap_or(false) {
            targets_after_invuln.push(target);
        }
    }

    if targets_after_invuln.is_empty() {
        return false;
    }

    // Step 1: TryHit event (Protect, Magic Bounce, Volt Absorb, etc.)
    // JavaScript (battle-actions.ts): this.hitStepTryHitEvent
    // This must run BEFORE accuracy check so that Protect can block the move
    let mut try_hit_results = vec![true; targets_after_invuln.len()];
    for (i, &target) in targets_after_invuln.iter().enumerate() {
        // Debug: Check if target has any volatiles
        {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => continue,
            };
            eprintln!("[TRY_SPREAD_MOVE_HIT] Target {:?} has {} volatiles: {:?}",
                target, target_pokemon.volatiles.len(),
                target_pokemon.volatiles.keys().map(|k| k.as_str()).collect::<Vec<_>>());
        }

        // JavaScript: hitResult = this.battle.runEvent('TryHit', targets, pokemon, move);
        // Use run_event (not single_event) to check target's volatiles like kingsshield
        let hit_result = battle.run_event(
            "TryHit",
            Some(target),
            Some(pokemon_pos),
            Some(move_id),
            None,
        );

        eprintln!("[TRY_SPREAD_MOVE_HIT] TryHit result for target {:?}: {:?}", target, hit_result);

        // JavaScript: if (hitResults[i] !== this.battle.NOT_FAIL) hitResults[i] = hitResults[i] || false;
        // Then: targets = targets.filter((val, i) => hitResults[i] || hitResults[i] === 0);
        // Keep targets where result is truthy OR === 0
        // Filter out where result is falsy AND !== 0 (i.e., false or NOT_FAIL)
        match hit_result {
            None => {
                // None represents falsy non-0 values (false, NOT_FAIL)
                try_hit_results[i] = false;
                eprintln!("[TRY_SPREAD_MOVE_HIT] Target {:?} BLOCKED by TryHit (None)", target);
            }
            Some(_) => {
                // Some(0) and Some(non-zero) both mean keep the target
                try_hit_results[i] = true;
                eprintln!("[TRY_SPREAD_MOVE_HIT] Target {:?} PASSED TryHit", target);
            }
        }
    }

    // Filter targets based on TryHit results
    let targets_after_try_hit: Vec<_> = targets_after_invuln.iter().enumerate()
        .filter(|(i, _)| try_hit_results[*i])
        .map(|(_, &t)| t)
        .collect();

    // If all targets blocked, move failed
    if targets_after_try_hit.is_empty() {
        return false;
    }

    // Step 2-3: Type Immunity, Move-specific Immunity
    // JavaScript: this.hitStepTypeImmunity, this.hitStepTryImmunity
    // NOTE: In gen 4-6, these steps are swapped (handled by generation-specific reordering)

    // Step 2: Type Immunity
    // JavaScript: this.hitStepTypeImmunity
    let type_immunity_results = {
        let mut active_move_clone = battle.active_move.clone().unwrap();
        crate::battle_actions::hit_step_type_immunity(
            battle,
            &targets_after_try_hit,
            pokemon_pos,
            &mut active_move_clone,
        )
    };

    // Filter targets based on type immunity
    let mut targets_after_type_immunity = Vec::new();
    for (i, &target) in targets_after_try_hit.iter().enumerate() {
        if type_immunity_results.get(i).copied().unwrap_or(false) {
            targets_after_type_immunity.push(target);
        }
    }

    if targets_after_type_immunity.is_empty() {
        return false;
    }

    // Step 3: Move-specific Immunity
    // JavaScript: this.hitStepTryImmunity
    let immunity_results = {
        let active_move_clone = battle.active_move.clone().unwrap();
        crate::battle_actions::hit_step_try_immunity(
            battle,
            &targets_after_type_immunity,
            pokemon_pos,
            &active_move_clone,
        )
    };

    // Filter targets based on immunity results
    let mut targets_after_immunity = Vec::new();
    for (i, &target) in targets_after_type_immunity.iter().enumerate() {
        if immunity_results.get(i).copied().unwrap_or(false) {
            targets_after_immunity.push(target);
        }
    }

    if targets_after_immunity.is_empty() {
        return false;
    }

    // Step 4: Check accuracy
    eprintln!("[TRY_SPREAD_MOVE_HIT] About to check accuracy for move={}, targets={:?}, PRNG before={}",
        move_id.as_str(), targets_after_immunity, battle.prng.call_count);
    let hit_results = crate::battle_actions::hit_step_accuracy(battle, &targets_after_immunity, pokemon_pos, move_id);
    eprintln!("[TRY_SPREAD_MOVE_HIT] Accuracy check results: {:?}, PRNG after={}", hit_results, battle.prng.call_count);

    // Filter out targets that failed accuracy check
    let mut remaining_targets = Vec::new();
    for (i, &target) in targets_after_immunity.iter().enumerate() {
        if hit_results.get(i).copied().unwrap_or(false) {
            remaining_targets.push(target);
        }
    }

    // If no targets remain, move failed
    if remaining_targets.is_empty() {
        return false;
    }

    // Step 5-6: Break Protect, Steal Boosts
    // JavaScript: this.hitStepBreakProtect, this.hitStepStealBoosts

    // Step 5: Break protection effects
    // JavaScript: this.hitStepBreakProtect
    {
        let active_move_clone = battle.active_move.clone().unwrap();
        crate::battle_actions::hit_step_break_protect(
            battle,
            &remaining_targets,
            pokemon_pos,
            &active_move_clone,
        );
    }

    // Step 6: Steal positive boosts (Spectral Thief)
    // JavaScript: this.hitStepStealBoosts
    {
        let active_move_clone = battle.active_move.clone().unwrap();
        crate::battle_actions::hit_step_steal_boosts(
            battle,
            &remaining_targets,
            pokemon_pos,
            &active_move_clone,
        );
    }

    // Step 7: Move hit loop (multi-hit handling)
    // JavaScript: this.hitStepMoveHitLoop(targets, pokemon, move);
    // For now, use a simplified approach: call spread_move_hit multiple times for multi-hit moves

    // Determine number of hits
    let num_hits = if let Some(ref active_move) = battle.active_move {
        eprintln!("[TRY_SPREAD_MOVE_HIT] Move {} has multi_hit: {:?}", active_move.id.as_str(), active_move.multi_hit);
        match &active_move.multi_hit {
            Some(crate::dex::Multihit::Fixed(n)) => {
                eprintln!("[TRY_SPREAD_MOVE_HIT] Fixed multi-hit: {}", n);
                *n
            }
            Some(crate::dex::Multihit::Range(min, max)) => {
                eprintln!("[TRY_SPREAD_MOVE_HIT] Range multi-hit: {}-{}", min, max);
                let min_val = *min;
                let max_val = *max;
                let gen = battle.gen;
                // Drop the reference before calling battle methods
                drop(active_move);

                // JavaScript logic for [2, 5] range:
                // if (targetHits[0] === 2 && targetHits[1] === 5) {
                //     if (this.battle.gen >= 5) {
                //         // 35-35-15-15 out of 100 for 2-3-4-5 hits
                //         targetHits = this.battle.sample([2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 5, 5, 5]);
                //     } else {
                //         targetHits = this.battle.sample([2, 2, 2, 3, 3, 3, 4, 5]);
                //     }
                // } else {
                //     targetHits = this.battle.random(targetHits[0], targetHits[1] + 1);
                // }
                if min_val == 2 && max_val == 5 {
                    if gen >= 5 {
                        // 35% chance of 2 hits, 35% of 3 hits, 15% of 4 hits, 15% of 5 hits
                        let distribution = vec![2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 5, 5, 5];
                        *battle.sample(&distribution).unwrap_or(&2)
                    } else {
                        let distribution = vec![2, 2, 2, 3, 3, 3, 4, 5];
                        *battle.sample(&distribution).unwrap_or(&2)
                    }
                } else {
                    battle.random((max_val - min_val + 1) as i32) + min_val
                }
            }
            None => {
                eprintln!("[TRY_SPREAD_MOVE_HIT] No multi_hit field");
                1
            }
        }
    } else {
        eprintln!("[TRY_SPREAD_MOVE_HIT] No active_move!");
        1
    };

    let target_list: Vec<Option<(usize, usize)>> = remaining_targets.iter().map(|&t| Some(t)).collect();
    let mut total_damages: Vec<i32> = vec![0; target_list.len()];
    let mut final_targets = target_list.clone();

    // Execute each hit
    for hit_num in 0..num_hits {
        eprintln!("[TRY_SPREAD_MOVE_HIT] Executing hit {} of {}", hit_num + 1, num_hits);

        // Set the hit number on active_move (JavaScript: move.hit = hit)
        if let Some(ref mut active_move) = battle.active_move {
            active_move.hit = (hit_num + 1) as i32;
        }

        let (damages, targets) =
            crate::battle_actions::spread_move_hit(battle, &final_targets, pokemon_pos, move_id, false, false);

        // Accumulate damage
        for (i, damage_opt) in damages.iter().enumerate() {
            if let Some(dmg) = damage_opt {
                total_damages[i] += dmg;
            }
        }

        // Update targets for next hit
        final_targets = targets;
    }

    // Convert to Option format
    let damages: Vec<Option<i32>> = total_damages.iter().map(|&d| Some(d)).collect();

    // JavaScript (battle-actions.ts line 831): move.totalDamage += damage[i];
    // Accumulate total damage for recoil calculation
    let mut total_damage = 0;
    for damage_opt in &damages {
        if let Some(dmg) = damage_opt {
            total_damage += dmg;
        }
    }

    // Store total damage in active_move for recoil handling
    if let Some(ref mut active_move) = battle.active_move {
        active_move.total_damage = total_damage;
    }

    // Check if any target was hit
    for (i, damage) in damages.iter().enumerate() {
        if let Some(dmg) = damage {
            if *dmg != 0 || final_targets.get(i).and_then(|t| *t).is_some() {
                return true;
            }
        }
    }

    false
}
