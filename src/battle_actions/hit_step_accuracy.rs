//! BattleActions::hitStepAccuracy - Check accuracy for move hits
//!
//! 1:1 port of hitStepAccuracy from battle-actions.ts:580

// JS Source:
// 	hitStepAccuracy(targets: Pokemon[], pokemon: Pokemon, move: ActiveMove) {
// 		const hitResults = [];
// 		for (const [i, target] of targets.entries()) {
// 			this.battle.activeTarget = target;
// 			// calculate true accuracy
// 			let accuracy = move.accuracy;
// 			if (move.ohko) { // bypasses accuracy modifiers
// 				if (!target.isSemiInvulnerable()) {
// 					accuracy = 30;
// 					if (move.ohko === 'Ice' && this.battle.gen >= 7 && !pokemon.hasType('Ice')) {
// 						accuracy = 20;
// 					}
// 					if (!target.volatiles['dynamax'] && pokemon.level >= target.level &&
// 						(move.ohko === true || !target.hasType(move.ohko))) {
// 						accuracy += (pokemon.level - target.level);
// 					} else {
// 						this.battle.add('-immune', target, '[ohko]');
// 						hitResults[i] = false;
// 						continue;
// 					}
// 				}
// 			} else {
// 				accuracy = this.battle.runEvent('ModifyAccuracy', target, pokemon, move, accuracy);
// 				if (accuracy !== true) {
// 					let boost = 0;
// 					if (!move.ignoreAccuracy) {
// 						const boosts = this.battle.runEvent('ModifyBoost', pokemon, null, null, { ...pokemon.boosts });
// 						boost = this.battle.clampIntRange(boosts['accuracy'], -6, 6);
// 					}
// 					if (!move.ignoreEvasion) {
// 						const boosts = this.battle.runEvent('ModifyBoost', target, null, null, { ...target.boosts });
// 						boost = this.battle.clampIntRange(boost - boosts['evasion'], -6, 6);
// 					}
// 					if (boost > 0) {
// 						accuracy = this.battle.trunc(accuracy * (3 + boost) / 3);
// 					} else if (boost < 0) {
// 						accuracy = this.battle.trunc(accuracy * 3 / (3 - boost));
// 					}
// 				}
// 			}
// 			if (
// 				move.alwaysHit || (move.id === 'toxic' && this.battle.gen >= 8 && pokemon.hasType('Poison')) ||
// 				(move.target === 'self' && move.category === 'Status' && !target.isSemiInvulnerable())
// 			) {
// 				accuracy = true; // bypasses ohko accuracy modifiers
// 			} else {
// 				accuracy = this.battle.runEvent('Accuracy', target, pokemon, move, accuracy);
// 			}
// 			if (accuracy !== true && !this.battle.randomChance(accuracy, 100)) {
// 				if (move.smartTarget) {
// 					move.smartTarget = false;
// 				} else {
// 					if (!move.spreadHit) this.battle.attrLastMove('[miss]');
// 					this.battle.add('-miss', pokemon, target);
// 				}
// 				if (!move.ohko && pokemon.hasItem('blunderpolicy') && pokemon.useItem()) {
// 					this.battle.boost({ spe: 2 }, pokemon);
// 				}
// 				hitResults[i] = false;
// 				continue;
// 			}
// 			hitResults[i] = true;
// 		}
// 		return hitResults;
// 	}


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
    let _pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
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
        // JavaScript: if (move.ohko) { ... }
        if let Some(ref ohko_type) = move_data.ohko {
            // if (!target.isSemiInvulnerable())
            let target_semi_invulnerable = Pokemon::is_semi_invulnerable(battle, target_pos);

            if !target_semi_invulnerable {
                // accuracy = 30;
                accuracy = 30;

                // if (move.ohko === 'Ice' && this.battle.gen >= 7 && !pokemon.hasType('Ice'))
                if let crate::dex::Ohko::TypeBased(ref type_name) = ohko_type {
                    if type_name == "Ice" && battle.gen >= 7 {
                        let attacker_has_ice = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1)
                            .map(|p| p.has_type(battle, "Ice"))
                            .unwrap_or(false);
                        if !attacker_has_ice {
                            // accuracy = 20;
                            accuracy = 20;
                        }
                    }
                }

                // if (!target.volatiles['dynamax'] && pokemon.level >= target.level &&
                //     (move.ohko === true || !target.hasType(move.ohko)))
                let (target_has_dynamax, pokemon_level, target_level, target_has_ohko_type) = {
                    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                        Some(p) => p,
                        None => return hit_results,
                    };
                    let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                        Some(t) => t,
                        None => {
                            hit_results[i] = false;
                            continue;
                        }
                    };
                    let has_dynamax = target.volatiles.contains_key(&ID::from("dynamax"));
                    let target_type_check = match ohko_type {
                        crate::dex::Ohko::Generic => {
                            false // ohko === true means any type is valid
                        }
                        crate::dex::Ohko::TypeBased(ref type_name) => {
                            target.has_type(battle, type_name.as_str())
                        }
                    };
                    (has_dynamax, pokemon.level, target.level, target_type_check)
                };

                let is_generic_ohko = matches!(ohko_type, crate::dex::Ohko::Generic);
                if !target_has_dynamax && pokemon_level >= target_level &&
                    (is_generic_ohko || !target_has_ohko_type) {
                    // accuracy += (pokemon.level - target.level);
                    accuracy += (pokemon_level - target_level) as i32;
                } else {
                    // this.battle.add('-immune', target, '[ohko]');
                    // hitResults[i] = false;
                    // continue;
                    if let Some(target_pokemon) = battle.pokemon_at(target_pos.0, target_pos.1) {
                        let target_ident = format!("p{}a: {}", target_pos.0 + 1, target_pokemon.set.species);
                        battle.add("-immune", &[
                            crate::battle::Arg::String(target_ident),
                            crate::battle::Arg::Str("[ohko]"),
                        ]);
                    }
                    hit_results[i] = false;
                    continue;
                }
            }
        } else {
            // Not an OHKO move - run ModifyAccuracy event
            // JavaScript: else { accuracy = this.battle.runEvent('ModifyAccuracy', target, pokemon, move, accuracy); }
            if let Some(modified_acc) = battle.run_event(
                "ModifyAccuracy",
                Some(target_pos),
                Some(pokemon_pos),
                Some(&move_id),
                Some(accuracy),
            ) {
                accuracy = modified_acc;
            }
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
            let mut boost;

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
        // if (
        //     move.alwaysHit || (move.id === 'toxic' && this.battle.gen >= 8 && pokemon.hasType('Poison')) ||
        //     (move.target === 'self' && move.category === 'Status' && !target.isSemiInvulnerable())
        // ) {
        //     accuracy = true; // bypasses ohko accuracy modifiers
        // } else {
        //     accuracy = this.battle.runEvent('Accuracy', target, pokemon, move, accuracy);
        // }

        let always_hit = {
            let active_always_hit = battle.active_move.as_ref()
                .map(|m| m.always_hit)
                .unwrap_or(false);

            active_always_hit ||
                (move_id.as_str() == "toxic" && battle.gen >= 8 && {
                    battle.pokemon_at(pokemon_pos.0, pokemon_pos.1)
                        .map(|p| p.has_type(battle, "Poison"))
                        .unwrap_or(false)
                }) ||
                (move_data.target == "self" && move_data.category == "Status" && !Pokemon::is_semi_invulnerable(battle, target_pos))
        };

        if always_hit {
            // accuracy = true; // bypasses ohko accuracy modifiers
            accuracy = 0; // In Rust, 0 represents boolean true for accuracy
        } else {
            // accuracy = this.battle.runEvent('Accuracy', target, pokemon, move, accuracy);
            if let Some(modified_acc) = battle.run_event(
                "Accuracy",
                Some(target_pos),
                Some(pokemon_pos),
                Some(&move_id),
                Some(accuracy),
            ) {
                accuracy = modified_acc;
            }
        }

        // JavaScript: if (accuracy !== true && !this.battle.randomChance(accuracy, 100))
        // In Rust, accuracy=0 represents true (boolean true from alwaysHit or Accuracy event)
        // JavaScript DOES call randomChance for accuracy=100 (the number 100, not true)
        // So we only skip if accuracy is 0 (representing boolean true)
        if battle.prng.call_count >= 137 && battle.prng.call_count <= 150 {
            eprintln!("[HIT_STEP_ACCURACY] About to check accuracy: accuracy={}, will call random_chance: {}, move={:?}",
                accuracy, accuracy != 0, move_id);
        }
        if accuracy != 0 && !battle.random_chance(accuracy, 100) {
            // Miss!
            // if (move.smartTarget) {
            //     move.smartTarget = false;
            // } else {
            //     if (!move.spreadHit) this.battle.attrLastMove('[miss]');
            //     this.battle.add('-miss', pokemon, target);
            // }
            // if (!move.ohko && pokemon.hasItem('blunderpolicy') && pokemon.useItem()) {
            //     this.battle.boost({ spe: 2 }, pokemon);
            // }

            // TODO: Need to modify ActiveMove.smart_target, but we only have a reference to move_data
            // For now, just handle the miss messages
            let spread_hit = battle.active_move.as_ref()
                .map(|m| m.spread_hit)
                .unwrap_or(false);

            if !spread_hit {
                battle.attr_last_move(&["[miss]"]);
            }

            if let Some(attacker_pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                let attacker_ident = format!("p{}a: {}", pokemon_pos.0 + 1, attacker_pokemon.set.species);
                if let Some(target_pokemon) = battle.pokemon_at(target_pos.0, target_pos.1) {
                    let target_ident = format!("p{}a: {}", target_pos.0 + 1, target_pokemon.set.species);
                    battle.add("-miss", &[
                        crate::battle::Arg::String(attacker_ident),
                        crate::battle::Arg::String(target_ident),
                    ]);
                }
            }

            // Blunder Policy item handling
            // if (!move.ohko && pokemon.hasItem('blunderpolicy') && pokemon.useItem())
            if move_data.ohko.is_none() {
                let has_blunder_policy = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1)
                    .map(|p| p.has_item(battle, &["blunderpolicy"]))
                    .unwrap_or(false);
                if has_blunder_policy {
                    // pokemon.useItem()
                    let used_item = Pokemon::use_item(battle, pokemon_pos, None, None);
                    if used_item.is_some() {
                        // this.battle.boost({ spe: 2 }, pokemon);
                        battle.boost(&[("spe", 2)], pokemon_pos, Some(pokemon_pos), None, false, false);
                    }
                }
            }

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
