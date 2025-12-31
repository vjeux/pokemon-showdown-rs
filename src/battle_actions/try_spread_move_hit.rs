//! BattleActions::trySpreadMoveHit - Try to hit targets with a spread move
//!
//! 1:1 port of trySpreadMoveHit from battle-actions.ts:545

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

    // Implement the 7-step pipeline from JavaScript's trySpreadMoveHit
    // Step 0: Invulnerability
    // TODO: Implement invulnerability check (hitStepInvulnerabilityEvent)

    // Step 1: TryHit event (Protect, Magic Bounce, Volt Absorb, etc.)
    // JavaScript (battle-actions.ts): this.hitStepTryHitEvent
    // This must run BEFORE accuracy check so that Protect can block the move
    let mut try_hit_results = vec![true; targets.len()];
    for (i, &target) in targets.iter().enumerate() {
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
    let targets_after_try_hit: Vec<_> = targets.iter().enumerate()
        .filter(|(i, _)| try_hit_results[*i])
        .map(|(_, &t)| t)
        .collect();

    // If all targets blocked, move failed
    if targets_after_try_hit.is_empty() {
        return false;
    }

    // Step 2-3: Type Immunity, Move-specific Immunity
    // TODO: Implement these steps when needed

    // PrepareHit event - must be called BEFORE accuracy check
    // JavaScript (battle-actions.ts:587):
    //   this.battle.singleEvent("PrepareHit", move, {}, targets[0], pokemon, move) &&
    //   this.battle.runEvent("PrepareHit", pokemon, targets[0], move);

    // First, call move's onPrepareHit handler via single_event
    let prepare_hit_result = battle.single_event(
        "PrepareHit",
        move_id,
        Some(targets_after_try_hit[0]),
        Some(pokemon_pos),
        None,
    );

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
        Some(targets_after_try_hit[0]),
        Some(move_id),
        None,
    );

    // Step 4: Check accuracy
    let hit_results = crate::battle_actions::hit_step_accuracy(battle, &targets_after_try_hit, pokemon_pos, move_id);

    // Filter out targets that failed accuracy check
    let mut remaining_targets = Vec::new();
    for (i, &target) in targets_after_try_hit.iter().enumerate() {
        if hit_results.get(i).copied().unwrap_or(false) {
            remaining_targets.push(target);
        }
    }

    // If no targets remain, move failed
    if remaining_targets.is_empty() {
        return false;
    }

    // Step 5-6: Break Protect, Steal Boosts
    // TODO: Implement these steps when needed

    // Step 7: Move hit loop (damage calculation)
    let target_list: Vec<Option<(usize, usize)>> = remaining_targets.iter().map(|&t| Some(t)).collect();

    let (damages, final_targets) =
        crate::battle_actions::spread_move_hit(battle, &target_list, pokemon_pos, move_id, false, false);

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
