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
    if targets.is_empty() {
        return false;
    }

    // Implement the 7-step pipeline from JavaScript's trySpreadMoveHit
    // Step 0-3: Invulnerability, TryHit, Type Immunity, Move-specific Immunity
    // TODO: Implement these steps when needed

    // Step 4: Check accuracy - THIS WAS THE MISSING STEP!
    let hit_results = crate::battle_actions::hit_step_accuracy(battle, targets, pokemon_pos, move_id);

    // Filter out targets that failed accuracy check
    let mut remaining_targets = Vec::new();
    for (i, &target) in targets.iter().enumerate() {
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
