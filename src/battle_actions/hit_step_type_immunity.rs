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
    if active_move.ignore_immunity.is_none() {
        if active_move.category == "Status" {
            active_move.ignore_immunity = Some(crate::battle_actions::IgnoreImmunity::All);
        }
    }

    // const hitResults = [];
    // for (const i of targets.keys()) {
    //     hitResults[i] = targets[i].runImmunity(move, !move.smartTarget);
    // }
    let mut hit_results = Vec::new();
    for &target_pos in targets {
        let hit_result = Pokemon::run_immunity(
            battle,
            target_pos,
            active_move.id.as_str(),
            !active_move.smart_target.unwrap_or(false),
        );
        hit_results.push(hit_result);
    }

    // return hitResults;
    hit_results
}
