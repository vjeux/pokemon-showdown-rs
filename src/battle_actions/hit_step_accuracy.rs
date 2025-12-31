//! BattleActions::hitStepAccuracy - Check accuracy for move hits
//!
//! 1:1 port of hitStepAccuracy from battle-actions.ts:580

use crate::*;

/// Check accuracy for each target
/// Equivalent to hitStepAccuracy() in battle-actions.ts:580
///
/// Returns a vec of booleans indicating whether each target was hit
pub fn hit_step_accuracy(
    battle: &mut Battle,
    targets: &[(usize, usize)],
    pokemon_pos: (usize, usize),
    move_id: &ID,
) -> Vec<bool> {
    let mut hit_results = vec![false; targets.len()];

    // Get move data
    let move_data = match battle.dex.moves().get(move_id.as_str()) {
        Some(m) => m.clone(),
        None => {
            // If move doesn't exist, consider all targets hit (shouldn't happen)
            return vec![true; targets.len()];
        }
    };

    // Get pokemon
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => {
            return hit_results;
        }
    };

    for (i, &target_pos) in targets.iter().enumerate() {
        // Get base accuracy from move
        let mut accuracy = match move_data.accuracy {
            crate::dex::Accuracy::Percent(p) => p,
            crate::dex::Accuracy::AlwaysHits => {
                hit_results[i] = true;
                continue;
            }
        };

        // Handle OHKO moves
        if move_data.ohko.is_some() {
            // TODO: Full OHKO logic
            // For now, treat as normal accuracy check
        }

        // Phase 1: Extract data immutably
        let (attacker_acc_boost, target_eva_boost) = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return hit_results,
            };
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(t) => t,
                None => continue,
            };
            (pokemon.boosts.accuracy, target.boosts.evasion)
        };

        // Phase 2: Mutate battle
        battle.active_target = Some(target_pos);

        // Apply accuracy/evasion boosts
        // Simplified version - just use accuracy/evasion boosts without checking flags
        if accuracy != 0 && accuracy != 100 {
            let mut boost = 0;

            // Get attacker's accuracy boost
            boost = attacker_acc_boost.max(-6).min(6);

            // Subtract target's evasion boost
            boost = (boost - target_eva_boost).max(-6).min(6);

            // Apply boost to accuracy
            if boost > 0 {
                accuracy = battle.trunc(accuracy as f64 * (3.0 + boost as f64) / 3.0, None) as i32;
            } else if boost < 0 {
                accuracy = battle.trunc(accuracy as f64 * 3.0 / (3.0 - boost as f64), None) as i32;
            }
        }

        // Check accuracy with randomChance
        // THIS IS THE KEY PRNG CALL THAT WAS MISSING!
        // JavaScript: if (accuracy !== true && !this.battle.randomChance(accuracy, 100))
        if accuracy != 0 && accuracy != 100 && !battle.random_chance(accuracy, 100) {
            // Miss!
            // TODO: Add miss message and Blunder Policy handling
            hit_results[i] = false;
            continue;
        }

        // Hit!
        hit_results[i] = true;
    }

    hit_results
}
