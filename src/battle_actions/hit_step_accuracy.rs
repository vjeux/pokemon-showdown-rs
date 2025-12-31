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
    eprintln!("[HIT_STEP_ACCURACY] Called for move {:?} from {:?} targeting {:?}", move_id, pokemon_pos, targets);
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
        eprintln!("[HIT_STEP_ACCURACY] Processing target {} of {}: {:?}", i, targets.len(), target_pos);
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

        // Set active_target for events
        battle.active_target = Some(target_pos);

        // JavaScript: else { accuracy = this.battle.runEvent('ModifyAccuracy', target, pokemon, move, accuracy); }
        // ModifyAccuracy event can change accuracy to true (represented as 0 in Rust)
        if let Some(modified_acc) = battle.run_event(
            "ModifyAccuracy",
            Some(target_pos),
            Some(pokemon_pos),
            Some(&move_id),
            Some(accuracy),
        ) {
            accuracy = modified_acc;
        }

        // JavaScript: if (accuracy !== true) { apply boosts }
        // In Rust, accuracy=0 represents true
        if accuracy != 0 {
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

            // Apply accuracy/evasion boosts
            // Simplified version - just use accuracy/evasion boosts without checking flags
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

        // JavaScript: if (move.alwaysHit || ...) { accuracy = true; } else { accuracy = runEvent('Accuracy', ...); }
        // Check special conditions that set accuracy to true
        // TODO: Implement move.target === 'self' and toxic special cases
        // For now, just run the Accuracy event
        if let Some(modified_acc) = battle.run_event(
            "Accuracy",
            Some(target_pos),
            Some(pokemon_pos),
            Some(&move_id),
            Some(accuracy),
        ) {
            accuracy = modified_acc;
        }

        // JavaScript: if (accuracy !== true && !this.battle.randomChance(accuracy, 100))
        // In Rust, accuracy=0 represents true (boolean true from alwaysHit or Accuracy event)
        // JavaScript DOES call randomChance for accuracy=100 (the number 100, not true)
        // So we only skip if accuracy is 0 (representing boolean true)
        eprintln!("[HIT_STEP_ACCURACY] About to check accuracy: accuracy={}, will call random_chance: {}", accuracy, accuracy != 0);
        if accuracy != 0 && !battle.random_chance(accuracy, 100) {
            // Miss!
            // TODO: Add miss message and Blunder Policy handling
            eprintln!("[HIT_STEP_ACCURACY] Miss! accuracy check failed");
            hit_results[i] = false;
            continue;
        }

        // Hit!
        eprintln!("[HIT_STEP_ACCURACY] Hit! target {} succeeded", i);
        hit_results[i] = true;
    }

    eprintln!("[HIT_STEP_ACCURACY] Returning results: {:?}", hit_results);
    hit_results
}
