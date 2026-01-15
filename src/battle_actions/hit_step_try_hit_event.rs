//! BattleActions::hitStepTryHitEvent - Fire TryHit event for targets
//!
//! 1:1 port of hitStepTryHitEvent from battle-actions.ts

use crate::*;
use crate::event::EventResult;
use crate::battle_actions::ActiveMove;
use crate::battle_actions::HitResult;
use crate::battle::Effect;

/// Fire TryHit event and handle failures
/// Equivalent to battle-actions.ts hitStepTryHitEvent()
///
/// hitStepTryHitEvent(targets: Pokemon[], pokemon: Pokemon, move: ActiveMove) {
///     const hitResults = this.battle.runEvent('TryHit', targets, pokemon, move);
///     if (!hitResults.includes(true) && hitResults.includes(false)) {
///         this.battle.add('-fail', pokemon);
///         this.battle.attrLastMove('[still]');
///     }
///     for (const i of targets.keys()) {
///         if (hitResults[i] !== this.battle.NOT_FAIL) hitResults[i] = hitResults[i] || false;
///     }
///     return hitResults;
/// }

pub fn hit_step_try_hit_event(
    battle: &mut Battle,
    targets: &[(usize, usize)],
    attacker_pos: (usize, usize),
    active_move: &ActiveMove,
) -> Vec<HitResult> {
    // const hitResults = this.battle.runEvent('TryHit', targets, pokemon, move);
    let mut hit_results = Vec::new();
    for &target_pos in targets {
        let result = battle.run_event("TryHit", Some(crate::event::EventTarget::Pokemon(target_pos)), Some(attacker_pos), Some(&Effect::move_(active_move.id.clone())), EventResult::Continue, false, false);
        hit_results.push(result);
    }

    // if (!hitResults.includes(true) && hitResults.includes(false)) {
    //     this.battle.add('-fail', pokemon);
    //     this.battle.attrLastMove('[still]');
    // }
    let has_true = hit_results.iter().any(|r| !matches!(r, EventResult::Number(0) | EventResult::Boolean(false) | EventResult::Null | EventResult::NotFail));
    let has_false = hit_results.iter().any(|r| matches!(r, EventResult::Number(0)));

    if !has_true && has_false {
        if let Some(attacker_pokemon) = battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
            let attacker_ident = format!("p{}a: {}", attacker_pos.0 + 1, attacker_pokemon.set.species);
            battle.add("-fail", &[
                crate::battle::Arg::String(attacker_ident),
            ]);
        }
        battle.attr_last_move(&["[still]"]);
    }

    // for (const i of targets.keys()) {
    //     if (hitResults[i] !== this.battle.NOT_FAIL) hitResults[i] = hitResults[i] || false;
    // }
    // return hitResults;

    // Convert EventResult to HitResult
    // JavaScript semantics:
    // - If result is NOT_FAIL (''): leave as NOT_FAIL (blocks but not a failure)
    // - If result is truthy: Success
    // - If result is falsy but not NOT_FAIL: Failed
    //
    // Critical: NOT_FAIL must NOT count as "atLeastOneFailure" for Temper Flare
    hit_results.iter().map(|r| {
        match r {
            EventResult::NotFail => HitResult::NotFail,  // Blocked but not a failure
            EventResult::Number(0) | EventResult::Boolean(false) => HitResult::Failed,  // Explicitly failed
            EventResult::Null => HitResult::Failed,      // null is falsy in JavaScript (immunity)
            _ => HitResult::Success,                     // anything else is truthy (success)
        }
    }).collect()
}
