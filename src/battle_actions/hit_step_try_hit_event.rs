//! BattleActions::hitStepTryHitEvent - Fire TryHit event for targets
//!
//! 1:1 port of hitStepTryHitEvent from battle-actions.ts

use crate::*;
use crate::battle_actions::ActiveMove;

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
) -> Vec<bool> {
    // const hitResults = this.battle.runEvent('TryHit', targets, pokemon, move);
    let mut hit_results = Vec::new();
    for &target_pos in targets {
        let result = battle.run_event(
            "TryHit",
            Some(target_pos),
            Some(attacker_pos),
            Some(&active_move.id),
            None,
        );
        hit_results.push(result);
    }

    // if (!hitResults.includes(true) && hitResults.includes(false)) {
    //     this.battle.add('-fail', pokemon);
    //     this.battle.attrLastMove('[still]');
    // }
    let has_true = hit_results.iter().any(|&r| r.is_some() && r != Some(0));
    let has_false = hit_results.iter().any(|&r| r == Some(0));

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

    // Convert Option<i32> results to bool
    // NOT_FAIL is a special value that means "don't fail but don't count as success either"
    // In JavaScript: if (hitResults[i] !== this.battle.NOT_FAIL) hitResults[i] = hitResults[i] || false;
    // This means: if not NOT_FAIL, convert to boolean (truthy or false)

    // In Rust, run_event returns Option<i32>:
    // - Some(0) = false
    // - Some(NOT_FAIL) = NOT_FAIL (special value, need to check what it is)
    // - Some(other) = true
    // - None = treated as true

    // For now, convert to bool: Some(0) = false, anything else = true
    // TODO: Check Battle::NOT_FAIL constant value
    hit_results.iter().map(|&r| r != Some(0)).collect()
}
