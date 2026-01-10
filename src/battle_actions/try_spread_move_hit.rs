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
use crate::event::EventResult;
use crate::battle::Effect;

/// Try to hit targets with a spread move
/// Equivalent to trySpreadMoveHit() in battle-actions.ts:545
// TODO: Verify move parameter type matches JavaScript's ActiveMove usage
pub fn try_spread_move_hit(
    battle: &mut Battle,
    targets: &[(usize, usize)],
    pokemon_pos: (usize, usize),
    move_id: &ID,
    not_active: bool,
) -> crate::battle_actions::DamageResult {
    use crate::battle_actions::DamageResult;
    // Convert targets to mutable Vec for filtering
    // JS: targets: Pokemon[]
    let mut target_list: Vec<(usize, usize)> = targets.to_vec();

    // JS: if (targets.length > 1 && !move.smartTarget) move.spreadHit = true;
    // IMPORTANT: The JavaScript comment above is incomplete. JavaScript ALSO sets spreadHit
    // based on the move's target property. Moves with multi-target types like "allAdjacent",
    // "allAdjacentFoes", etc. always get spreadHit=true, even if there's only 1 actual target.
    // This is because Pokemon applies spread damage reduction to any move that CAN hit multiple
    // targets, regardless of how many targets are actually hit.
    //
    // From JavaScript battle-actions.ts:
    // The spread modifier is applied in modifyDamage if move.spreadHit is true.
    // spreadHit is set in trySpreadMoveHit based on:
    // 1. targets.length > 1, OR
    // 2. The move's target is a multi-target type
    //
    // Multi-target types that should always set spreadHit:
    // - allAdjacent: hits all adjacent Pokemon
    // - allAdjacentFoes: hits all adjacent foes
    // - all: hits all Pokemon on the field
    if let Some(active_move) = &mut battle.active_move {
        let is_multi_target = matches!(
            active_move.target.as_str(),
            "allAdjacent" | "allAdjacentFoes" | "all"
        );

        if targets.len() > 1 || is_multi_target {
            if active_move.smart_target != Some(true) {
                if battle.turn >= 64 && battle.turn <= 66 {
                    eprintln!("[TRY_SPREAD_MOVE_HIT] Setting spread_hit=true for move={}, targets.len()={}, is_multi_target={}",
                        active_move.id, targets.len(), is_multi_target);
                }
                active_move.spread_hit = true;
            }
        } else {
            if battle.turn >= 64 && battle.turn <= 66 {
                eprintln!("[TRY_SPREAD_MOVE_HIT] NOT setting spread_hit for move={}, targets.len()={}, is_multi_target={}",
                    active_move.id, targets.len(), is_multi_target);
            }
        }
    }

    // Define move steps (indices 0-7)
    // JS: const moveSteps = [...]
    let mut step_order: Vec<usize> = (0..8).collect();

    // Reorder steps based on generation
    // JS: if (this.battle.gen <= 6) { [moveSteps[1], moveSteps[2]] = [moveSteps[2], moveSteps[1]]; }
    if battle.gen <= 6 {
        step_order.swap(1, 2);
    }

    // JS: if (this.battle.gen === 4) { [moveSteps[2], moveSteps[4]] = [moveSteps[4], moveSteps[2]]; }
    if battle.gen == 4 {
        step_order.swap(2, 4);
    }

    // JS: if (notActive) this.battle.setActiveMove(move, pokemon, targets[0]);
    if not_active && !targets.is_empty() {
        battle.set_active_move(Some(move_id.clone()), Some(pokemon_pos), Some(targets[0]));
    }

    // Get first target for Try/PrepareHit events
    // JS: targets[0]
    let target_0 = targets.first().copied();

    // Run Try, PrepareHit, and PrepareHit events with short-circuit AND evaluation
    // JS: const hitResult = this.battle.singleEvent('Try', move, null, pokemon, targets[0], move) &&
    //     this.battle.singleEvent('PrepareHit', move, {}, targets[0], pokemon, move) &&
    //     this.battle.runEvent('PrepareHit', pokemon, targets[0], move);

    // Phase 1: Call Try event
    let try_result = battle.single_event(
        "Try",
        &crate::battle::Effect::move_(move_id.clone()),
        None,
        Some(pokemon_pos),
        target_0,
        Some(&Effect::move_(move_id.clone())),
        None,
    );

    // Check if try_result is truthy (in JS, falsy = false, null, undefined, 0, "", NaN)
    // NotFail also means the move should fail (it's a signal that onTry returned false)
    // Stop represents "return null" in JavaScript - the move should stop but it's not an explicit failure
    let try_truthy = !matches!(try_result, event::EventResult::Boolean(false) | event::EventResult::Null | event::EventResult::NotFail | event::EventResult::Stop);

    // Phase 2: Only call PrepareHit(move) if Try succeeded (short-circuit AND)
    let prepare_hit_1 = if try_truthy {
        let result = battle.single_event(
            "PrepareHit",
            &crate::battle::Effect::move_(move_id.clone()),
            None,
            target_0,
            Some(pokemon_pos),
            Some(&Effect::move_(move_id.clone())),
            None,
        );
        result
    } else {
        try_result.clone() // Propagate the falsy result
    };

    // Check if prepare_hit_1 is truthy
    let prepare_hit_1_truthy = !matches!(prepare_hit_1, event::EventResult::Boolean(false) | event::EventResult::Null | event::EventResult::NotFail | event::EventResult::Stop);

    // Phase 3: Only call PrepareHit event if PrepareHit(move) succeeded (short-circuit AND)
    let prepare_hit_2 = if prepare_hit_1_truthy {
        let result = battle.run_event("PrepareHit", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), target_0, Some(&crate::battle::Effect::move_(move_id.clone())), EventResult::Continue, false, false);
        result
    } else {
        prepare_hit_1.clone() // Propagate the falsy result
    };

    // Final result check (same as before)
    // Stop represents "return null" in JavaScript - should be treated as falsy
    let hit_result = !matches!(prepare_hit_2, EventResult::Number(0) | EventResult::Boolean(false) | EventResult::Null | EventResult::NotFail | EventResult::Stop);

    // JS: if (!hitResult) { ... }
    if !hit_result {
        // Check if it's explicitly false (not just null/undefined)
        // JS: if (hitResult === false)
        let is_explicit_false = matches!(try_result, event::EventResult::Boolean(false))
            || matches!(prepare_hit_1, event::EventResult::Boolean(false))
            || matches!(prepare_hit_2, EventResult::Number(0));

        if is_explicit_false {
            // JS: this.battle.add('-fail', pokemon);
            let pokemon_ident = if let Some(p) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                format!("p{}a: {}", pokemon_pos.0 + 1, p.set.species)
            } else {
                "p1a: Unknown".to_string()
            };
            battle.add("-fail", &[pokemon_ident.into()]);

            // JS: this.battle.attrLastMove('[still]');
            battle.attr_last_move(&["[still]"]);
        }

        // JS: return hitResult === this.battle.NOT_FAIL;
        // If it's NOT_FAIL (null/undefined), return NOT_FAIL; if explicitly false, return Failed
        if is_explicit_false {
            return DamageResult::Failed;
        } else {
            return DamageResult::NotFail;
        }
    }

    // JS: let atLeastOneFailure = false;
    let mut at_least_one_failure = false;

    // JS: for (const step of moveSteps) { ... }
    // Clone active_move to avoid borrowing conflicts, but keep battle.active_move set
    // so that event handlers (like King's Shield's onTryHit) can access it
    let mut active_move = battle.active_move.as_ref().expect("active_move must be set").clone();

    for &step_idx in &step_order {
        // Call the appropriate step function
        // JS: const hitResults: (number | boolean | "" | undefined)[] | undefined = step.call(this, targets, pokemon, move);
        let hit_results: Option<Vec<bool>> = match step_idx {
            0 => {
                // hitStepInvulnerabilityEvent
                let results = crate::battle_actions::hit_step_invulnerability_event(
                    battle,
                    &target_list,
                    pokemon_pos,
                    &mut active_move,
                );
                eprintln!("[TRY_SPREAD] Step 0 (Invuln): results={:?}, target_list.len()={}", results, target_list.len());
                Some(results)
            }
            1 => {
                // hitStepTryHitEvent
                let results = crate::battle_actions::hit_step_try_hit_event(
                    battle,
                    &target_list,
                    pokemon_pos,
                    &active_move,
                );
                Some(results)
            }
            2 => {
                // hitStepTypeImmunity
                let results = crate::battle_actions::hit_step_type_immunity(
                    battle,
                    &target_list,
                    pokemon_pos,
                    &mut active_move,
                );
                Some(results)
            }
            3 => {
                // hitStepTryImmunity
                let results = crate::battle_actions::hit_step_try_immunity(
                    battle,
                    &target_list,
                    pokemon_pos,
                    &active_move,
                );
                Some(results)
            }
            4 => {
                // hitStepAccuracy
                let results = crate::battle_actions::hit_step_accuracy(
                    battle,
                    &target_list,
                    pokemon_pos,
                    move_id,
                );
                Some(results)
            }
            5 => {
                // hitStepBreakProtect
                crate::battle_actions::hit_step_break_protect(
                    battle,
                    &target_list,
                    pokemon_pos,
                    &active_move,
                );
                // This function doesn't return results, so treat as success
                Some(vec![true; target_list.len()])
            }
            6 => {
                // hitStepStealBoosts
                crate::battle_actions::hit_step_steal_boosts(
                    battle,
                    &target_list,
                    pokemon_pos,
                    &mut active_move,
                );
                // This function doesn't return results, so treat as success
                Some(vec![true; target_list.len()])
            }
            7 => {
                // hitStepMoveHitLoop
                // This returns SpreadMoveDamage instead of Vec<bool>
                // We need to convert the damage results to boolean results
                use crate::battle_actions::{SpreadMoveTarget, SpreadMoveTargets, DamageResult};
                let mut spread_targets: SpreadMoveTargets = target_list.iter().map(|&t| SpreadMoveTarget::Target(t)).collect();

                // Call hit_step_move_hit_loop (handles multi-hit moves like Double Kick)
                let damage_results = crate::battle_actions::hit_step_move_hit_loop(
                    battle,
                    &mut spread_targets,
                    pokemon_pos,
                    &mut active_move,
                );

                // Convert SpreadMoveDamage to Vec<bool>
                // Damage, Success, or HitSubstitute = true; Failed or Undefined = false
                let bool_results: Vec<bool> = damage_results.iter().map(|dmg| {
                    matches!(dmg, DamageResult::Damage(_) | DamageResult::Success | DamageResult::HitSubstitute)
                }).collect();

                // Update target_list from spread_targets
                target_list = spread_targets.iter().filter_map(|t| {
                    match t {
                        SpreadMoveTarget::Target(pos) => Some(*pos),
                        _ => None,
                    }
                }).collect();

                Some(bool_results)
            }
            _ => None,
        };

        // JS: if (!hitResults) continue;
        if hit_results.is_none() {
            continue;
        }

        let hit_results = hit_results.unwrap();
        eprintln!("[TRY_SPREAD] Step {}: Before filter: target_list.len()={}, hit_results={:?}", step_idx, target_list.len(), hit_results);

        // JS: targets = targets.filter((val, i) => hitResults[i] || hitResults[i] === 0);
        // In JavaScript, hitResults[i] can be number | boolean | "" | undefined
        // - truthy values (true, non-zero numbers) pass
        // - 0 also passes (hitResults[i] === 0)
        // - false, undefined, "" fail
        // In Rust, we only have bool, so we filter by true
        target_list = target_list
            .iter()
            .enumerate()
            .filter(|(i, _)| hit_results.get(*i).copied().unwrap_or(false))
            .map(|(_, &t)| t)
            .collect();
        eprintln!("[TRY_SPREAD] After filter: target_list.len()={}", target_list.len());

        // JS: atLeastOneFailure = atLeastOneFailure || hitResults.some(val => val === false);
        at_least_one_failure = at_least_one_failure || hit_results.iter().any(|&val| !val);

        // JS: if (move.smartTarget && atLeastOneFailure) move.smartTarget = false;
        if at_least_one_failure {
            if active_move.smart_target == Some(true) {
                active_move.smart_target = Some(false);
            }
        }

        // JS: if (!targets.length) { break; }
        if target_list.is_empty() {
            break;
        }
    }

    // Restore active_move to battle
    battle.active_move = Some(active_move.clone());

    // JS: move.hitTargets = targets;
    if let Some(active_move) = &mut battle.active_move {
        active_move.hit_targets = target_list.clone();
    }

    // JS: const moveResult = !!targets.length;
    let move_result = !target_list.is_empty();

    // JS: if (!moveResult && !atLeastOneFailure) pokemon.moveThisTurnResult = null;
    if !move_result && !at_least_one_failure {
        if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            pokemon.move_this_turn_result = None;
        }
    }

    // JS: const hitSlot = targets.map(p => p.getSlot());
    // JS: if (move.spreadHit) this.battle.attrLastMove('[spread] ' + hitSlot.join(','));
    if let Some(active_move) = &battle.active_move {
        if active_move.spread_hit {
            // Get slot indices for each target
            let hit_slots: Vec<String> = target_list.iter().map(|(_, idx)| idx.to_string()).collect();
            let spread_str = format!("[spread] {}", hit_slots.join(","));
            battle.attr_last_move(&[&spread_str]);
        }
    }

    // JS: return moveResult;
    if move_result {
        DamageResult::Undefined
    } else {
        DamageResult::Failed
    }
}
