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
use crate::event::EventResult;
use crate::battle::Effect;
use crate::battle_actions::ActiveMove;

/// Check accuracy for each target
/// Equivalent to hitStepAccuracy() in battle-actions.ts:580
///
/// Returns a vec of booleans indicating whether each target was hit
pub fn hit_step_accuracy(
    battle: &mut Battle,
    targets: &[(usize, usize)],
    pokemon_pos: (usize, usize),
    active_move: &mut ActiveMove,
) -> Vec<bool> {
    let move_id = active_move.id.clone();
    debug_elog!("[HIT_STEP_ACCURACY] Called for move {:?} from {:?} targeting {:?}", move_id, pokemon_pos, targets);
    let mut hit_results = vec![false; targets.len()];

    // Get pokemon
    let _pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => {
            return hit_results;
        }
    };

    for (i, &target_pos) in targets.iter().enumerate() {
        debug_elog!("[HIT_STEP_ACCURACY] Processing target {} of {}: {:?}", i, targets.len(), target_pos);
        // Get base accuracy from move
        // Use the passed active_move parameter directly
        // JavaScript: let accuracy = move.accuracy;
        // NOTE: In JavaScript, accuracy can be:
        //   - A number (e.g., 100)
        //   - Boolean true (always hits)
        // IMPORTANT: `accuracy: true` is DIFFERENT from `alwaysHit: true`:
        //   - accuracy: true -> The move has 100% accuracy, but Accuracy event still runs
        //   - alwaysHit: true -> Skip the Accuracy event entirely
        // In Rust, we use 0 to represent boolean true for accuracy.
        // We do NOT early return here - the `always_hit` check later determines if the
        // Accuracy event is skipped.
        let mut accuracy = match active_move.accuracy {
            crate::dex::Accuracy::Percent(p) => p,
            crate::dex::Accuracy::AlwaysHits => 0, // 0 represents boolean true
        };
        // Track whether accuracy started as boolean true (for ModifyAccuracy handling)
        let accuracy_is_true = accuracy == 0;

        // Handle OHKO moves
        // JavaScript: if (move.ohko) { ... }
        if let Some(ref ohko_type) = active_move.ohko {
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
            if let EventResult::Number(modified_acc) = battle.run_event(
                "ModifyAccuracy",
                Some(crate::event::EventTarget::Pokemon(target_pos)),
                Some(pokemon_pos),
                Some(&Effect::move_(move_id.clone())),
                EventResult::Number(accuracy),
                false,
                false
            ) {
                accuracy = modified_acc;
            }

            // JavaScript: if (accuracy !== true) { apply boosts }
            // NOTE: This is inside the else block - OHKO moves bypass accuracy modifiers including boosts
            // In Rust, accuracy=0 represents true
            if accuracy != 0 {
                // JavaScript: let boost = 0;
                let mut boost = 0;

                // if (!move.ignoreAccuracy)
                if !active_move.ignore_accuracy {
                    // const boosts = this.battle.runEvent('ModifyBoost', pokemon, null, null, { ...pokemon.boosts });
                    // boost = this.battle.clampIntRange(boosts['accuracy'], -6, 6);

                    // Create boosts table from pokemon's boosts
                    let boosts_table = {
                        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                            Some(p) => p,
                            None => return hit_results,
                        };
                        pokemon.boosts.clone()
                    };

                    // Run ModifyBoost event - returns modified boosts table
                    let modified_boosts = battle.run_event(
                    "ModifyBoost",
                    Some(crate::event::EventTarget::Pokemon(pokemon_pos)),
                        None,
                        None,
                        crate::event::EventResult::Boost(boosts_table),
                        false,
                        false,
                    ).boost().unwrap_or(boosts_table);

                    // Extract accuracy boost and clamp to [-6, 6]
                    boost = modified_boosts.accuracy.max(-6).min(6);
                }

                // if (!move.ignoreEvasion)
                if !active_move.ignore_evasion {
                    // const boosts = this.battle.runEvent('ModifyBoost', target, null, null, { ...target.boosts });
                    // boost = this.battle.clampIntRange(boost - boosts['evasion'], -6, 6);

                    // Create boosts table from target's boosts
                    let boosts_table = {
                        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                            Some(t) => t,
                            None => continue,
                        };
                        target.boosts.clone()
                    };

                    // Run ModifyBoost event - returns modified boosts table
                    let modified_boosts = battle.run_event(
                    "ModifyBoost",
                    Some(crate::event::EventTarget::Pokemon(target_pos)),
                        None,
                        None,
                        crate::event::EventResult::Boost(boosts_table),
                        false,
                        false,
                    ).boost().unwrap_or(boosts_table);

                    // Extract evasion boost, subtract from accuracy boost, and clamp to [-6, 6]
                    boost = (boost - modified_boosts.evasion).max(-6).min(6);
                }

                // Apply boost to accuracy
                if boost > 0 {
                    accuracy = battle.trunc(accuracy as f64 * (3.0 + boost as f64) / 3.0, None) as i32;
                } else if boost < 0 {
                    accuracy = battle.trunc(accuracy as f64 * 3.0 / (3.0 - boost as f64), None) as i32;
                }
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
            active_move.always_hit ||
                (move_id.as_str() == "toxic" && battle.gen >= 8 && {
                    battle.pokemon_at(pokemon_pos.0, pokemon_pos.1)
                        .map(|p| p.has_type(battle, "Poison"))
                        .unwrap_or(false)
                }) ||
                (active_move.target == "self" && active_move.category == "Status" && !Pokemon::is_semi_invulnerable(battle, target_pos))
        };

        if always_hit {
            // accuracy = true; // bypasses ohko accuracy modifiers
            accuracy = 0; // In Rust, 0 represents boolean true for accuracy
        } else {
            // accuracy = this.battle.runEvent('Accuracy', target, pokemon, move, accuracy);
            let result = battle.run_event(
                "Accuracy",
                Some(crate::event::EventTarget::Pokemon(target_pos)),
                Some(pokemon_pos),
                Some(&Effect::move_(move_id.clone())),
                EventResult::Number(accuracy),
                false,
                false
            );
            // JavaScript: accuracy can be true (always hit) or a number
            // No Guard returns true from onAnyAccuracy, meaning always hit
            match result {
                EventResult::Boolean(true) => {
                    // true means always hit - set accuracy to 0 (our representation of true)
                    accuracy = 0;
                }
                EventResult::Number(modified_acc) => {
                    accuracy = modified_acc;
                }
                _ => {
                    // Keep existing accuracy
                }
            }
        }

        // JavaScript: if (accuracy !== true && !this.battle.randomChance(accuracy, 100))
        // In Rust, accuracy=0 represents true (boolean true from alwaysHit or Accuracy event)
        // We skip randomChance ONLY if accuracy is 0 (representing boolean true)
        // For numeric accuracy values like 100, we MUST call randomChance (because 100 !== true in JavaScript)
        if accuracy != 0 && !battle.random_chance(accuracy, 100) {
            // Miss!
            // JavaScript: if (move.smartTarget) {
            //     move.smartTarget = false;
            // } else {
            //     if (!move.spreadHit) this.battle.attrLastMove('[miss]');
            //     this.battle.add('-miss', pokemon, target);
            // }

            let was_smart = active_move.smart_target.unwrap_or(false);
            if was_smart {
                // If smart_target was true, set it to false and don't show miss message
                active_move.smart_target = Some(false);
            }
            let spread_hit = active_move.spread_hit;

            // Only show miss messages if it wasn't a smart target miss
            if !was_smart {
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
            }

            // Blunder Policy item handling
            // if (!move.ohko && pokemon.hasItem('blunderpolicy') && pokemon.useItem())
            if active_move.ohko.is_none() {
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

            debug_elog!("[HIT_STEP_ACCURACY] Miss! accuracy check failed");
            hit_results[i] = false;
            continue;
        }

        // Hit!
        debug_elog!("[HIT_STEP_ACCURACY] Hit! target {} succeeded", i);
        hit_results[i] = true;
    }

    debug_elog!("[HIT_STEP_ACCURACY] Returning results: {:?}", hit_results);
    hit_results
}
