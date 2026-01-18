//! BattleActions::hitStepTypeImmunity - Check type-based immunity
//!
//! 1:1 port of hitStepTypeImmunity from battle-actions.ts

use crate::*;
use crate::battle_actions::ActiveMove;

/// Check type immunity (e.g., Ground vs Electric)
/// Equivalent to battle-actions.ts hitStepTypeImmunity()
///
/// hitStepTypeImmunity(targets: Pokemon[], pokemon: Pokemon, move: ActiveMove) {
///     if (move.ignoreImmunity === undefined) {
///         move.ignoreImmunity = (move.category === 'Status');
///     }
///
///     const hitResults = [];
///     for (const i of targets.keys()) {
///         hitResults[i] = targets[i].runImmunity(move, !move.smartTarget);
///     }
///
///     return hitResults;
/// }

pub fn hit_step_type_immunity(
    battle: &mut Battle,
    targets: &[(usize, usize)],
    _attacker_pos: (usize, usize),
    active_move: &mut ActiveMove,
) -> Vec<bool> {
    // if (move.ignoreImmunity === undefined) {
    //     move.ignoreImmunity = (move.category === 'Status');
    // }
    debug_elog!("[HIT_STEP_TYPE_IMM] BEFORE: move={}, category={}, ignore_immunity={:?}",
        active_move.id, active_move.category, active_move.ignore_immunity);
    if active_move.ignore_immunity.is_none() {
        if active_move.category == "Status" {
            active_move.ignore_immunity = Some(crate::battle_actions::IgnoreImmunity::All);
            debug_elog!("[HIT_STEP_TYPE_IMM] Set ignore_immunity=All for Status move");
        }
    }
    debug_elog!("[HIT_STEP_TYPE_IMM] AFTER: ignore_immunity={:?}", active_move.ignore_immunity);

    // const hitResults = [];
    // for (const i of targets.keys()) {
    //     hitResults[i] = targets[i].runImmunity(move, !move.smartTarget);
    // }
    let mut hit_results = Vec::new();
    debug_elog!("[HIT_STEP_TYPE_IMM] move={}, move_type={}", active_move.id, active_move.move_type);
    for &target_pos in targets {
        // JavaScript runImmunity checks: if (source.ignoreImmunity && (source.ignoreImmunity === true || source.ignoreImmunity[type]))
        // Check if move should ignore immunity for this type
        let should_ignore_immunity = match &active_move.ignore_immunity {
            Some(crate::battle_actions::IgnoreImmunity::All) => true,
            Some(crate::battle_actions::IgnoreImmunity::Specific(map)) => {
                map.get(&active_move.move_type).copied().unwrap_or(false)
            },
            // NoIgnore explicitly means don't ignore immunity, same as None
            Some(crate::battle_actions::IgnoreImmunity::NoIgnore) => false,
            None => false,
        };
        debug_elog!("[HIT_STEP_TYPE_IMM] target={:?}, should_ignore={}", target_pos, should_ignore_immunity);

        let hit_result = if should_ignore_immunity {
            // Bypass immunity check - move hits regardless of type immunity
            true
        } else {
            // Normal immunity check
            let result = Pokemon::run_immunity(
                battle,
                target_pos,
                &active_move.move_type,
                !active_move.smart_target.unwrap_or(false),
            );
            debug_elog!("[HIT_STEP_TYPE_IMM] run_immunity returned: {}", result);
            result
        };
        hit_results.push(hit_result);
    }

    // return hitResults;
    hit_results
}
