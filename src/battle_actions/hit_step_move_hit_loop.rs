//! BattleActions::hitStepMoveHitLoop - Main loop for multi-hit moves
//!
//! 1:1 port of hitStepMoveHitLoop from battle-actions.ts

use crate::*;
use crate::event::EventResult;
use crate::battle_actions::{SpreadMoveDamage, DamageResult, SpreadMoveTargets, SpreadMoveTarget, ActiveMove};
use crate::dex::Multihit;
use crate::battle::Effect;

/// Main loop for handling multi-hit moves
/// Equivalent to battle-actions.ts hitStepMoveHitLoop()
///
/// hitStepMoveHitLoop(targets: Pokemon[], pokemon: Pokemon, move: ActiveMove) {
///     let damage: (number | boolean | undefined)[] = [];
///     for (const i of targets.keys()) {
///         damage[i] = 0;
///     }
///     move.totalDamage = 0;
///     pokemon.lastDamage = 0;
///     // ... (see full JS source in file header)
///     return damage;
/// }
pub fn hit_step_move_hit_loop(
    battle: &mut Battle,
    targets: &mut SpreadMoveTargets,
    attacker_pos: (usize, usize),
    active_move: &mut ActiveMove,
) -> SpreadMoveDamage {
    // let damage: (number | boolean | undefined)[] = [];
    // for (const i of targets.keys()) {
    //     damage[i] = 0;
    // }
    let mut damage: SpreadMoveDamage = vec![DamageResult::Damage(0); targets.len()];

    // move.totalDamage = 0;
    active_move.total_damage = 0;

    // pokemon.lastDamage = 0;
    if let Some(pokemon) = battle.pokemon_at_mut(attacker_pos.0, attacker_pos.1) {
        pokemon.last_damage = 0;
    }

    // let targetHits = move.multihit || 1;
    let mut target_hits = match &active_move.multi_hit {
        Some(Multihit::Fixed(n)) => *n,
        Some(Multihit::Range(min, max)) => {
            // if (Array.isArray(targetHits)) {
            // yes, it's hardcoded... meh
            // if (targetHits[0] === 2 && targetHits[1] === 5) {
            if *min == 2 && *max == 5 {
                if battle.gen >= 5 {
                    // 35-35-15-15 out of 100 for 2-3-4-5 hits
                    let distribution = vec![2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 5, 5, 5];
                    let mut hits = *battle.sample(&distribution).unwrap();
                    if hits < 4 {
                        let has_loaded_dice = battle.pokemon_at(attacker_pos.0, attacker_pos.1)
                            .map(|p| p.has_item(battle, &["loadeddice"]))
                            .unwrap_or(false);
                        if has_loaded_dice {
                            hits = 5 - battle.random(2) as i32;
                        }
                    }
                    hits
                } else {
                    let distribution = vec![2, 2, 2, 3, 3, 3, 4, 5];
                    *battle.sample(&distribution).unwrap()
                }
            } else {
                // For other ranges, just pick a random value in the range
                battle.random((*max - *min + 1) as i32) + *min
            }
        }
        None => 1,
    };

    // Special case for loaded dice with targetHits == 10
    // if (targetHits === 10 && pokemon.hasItem('loadeddice')) targetHits -= this.battle.random(7);
    if target_hits == 10 {
        let has_loaded_dice = battle.pokemon_at(attacker_pos.0, attacker_pos.1)
            .map(|p| p.has_item(battle, &["loadeddice"]))
            .unwrap_or(false);
        if has_loaded_dice {
            target_hits -= battle.random(7) as i32;
        }
    }

    // targetHits = Math.floor(targetHits);
    target_hits = target_hits.max(1); // Ensure at least 1 hit

    // let nullDamage = true;
    let mut null_damage = true;

    // let moveDamage: (number | boolean | undefined)[] = [];
    let mut move_damage: SpreadMoveDamage = Vec::new();

    // const isSleepUsable = move.sleepUsable || this.dex.moves.get(move.sourceEffect).sleepUsable;
    let is_sleep_usable = {
        let base_sleep_usable = active_move.sleep_usable;

        // Check if sourceEffect move also has sleepUsable
        if let Some(ref source_effect_id) = active_move.source_effect {
            if let Some(source_effect_move) = battle.dex.moves().get(source_effect_id.as_str()) {
                base_sleep_usable || source_effect_move.sleep_usable
            } else {
                base_sleep_usable
            }
        } else {
            base_sleep_usable
        }
    };

    // let targetsCopy: (Pokemon | false | null)[] = targets.slice(0);
    let mut targets_copy = targets.clone();

    // let hit: number;
    // for (hit = 1; hit <= targetHits; hit++) {
    let mut hit = 1;
    while hit <= target_hits {
        // if (damage.includes(false)) break;
        if damage.iter().any(|d| matches!(d, DamageResult::Failed)) {
            break;
        }

        // if (hit > 1 && pokemon.status === 'slp' && (!isSleepUsable || this.battle.gen === 4)) break;
        if hit > 1 {
            let is_asleep = battle.pokemon_at(attacker_pos.0, attacker_pos.1)
                .map(|p| p.status.as_str() == "slp")
                .unwrap_or(false);
            if is_asleep && (!is_sleep_usable || battle.gen == 4) {
                break;
            }
        }

        // if (targets.every(target => !target?.hp)) break;
        let all_targets_fainted = targets.iter().all(|t| {
            match t {
                SpreadMoveTarget::Target(pos) => {
                    battle.pokemon_at(pos.0, pos.1).map(|p| p.hp == 0).unwrap_or(true)
                }
                _ => true,
            }
        });
        if all_targets_fainted {
            break;
        }

        // move.hit = hit;
        active_move.hit = hit;
        // Also sync to battle.active_move so callbacks can read it
        if let Some(ref mut battle_active_move) = battle.active_move {
            battle_active_move.hit = hit;
        }

        // Handle smartTarget
        // if (move.smartTarget && targets.length > 1) {
        //     targetsCopy = [targets[hit - 1]];
        //     damage = [damage[hit - 1]];
        // } else {
        //     targetsCopy = targets.slice(0);
        // }
        if active_move.smart_target.unwrap_or(false) && targets.len() > 1 {
            if (hit as usize) <= targets.len() {
                targets_copy = vec![targets[(hit as usize) - 1].clone()];
                // Keep only the relevant damage entry
                let relevant_damage = damage.get((hit as usize) - 1).cloned()
                    .unwrap_or(DamageResult::Damage(0));
                damage = vec![relevant_damage];
            }
        } else {
            targets_copy = targets.clone();
        }

        // const target = targetsCopy[0];
        // Handle retargeting for smartTarget
        // if (target && typeof move.smartTarget === 'boolean') {
        //     if (hit > 1) {
        //         this.battle.addMove('-anim', pokemon, move.name, target);
        //     } else {
        //         this.battle.retargetLastMove(target);
        //     }
        // }
        if let Some(target_pos) = targets_copy.get(0) {
            if let SpreadMoveTarget::Target(target) = target_pos {
                // Check if smartTarget is explicitly set (typeof move.smartTarget === 'boolean')
                if active_move.smart_target.is_some() {
                    if hit > 1 {
                        // this.battle.addMove('-anim', pokemon, move.name, target);
                        let pokemon_str = {
                            let pokemon = match battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
                                Some(p) => p,
                                None => {
                                    hit += 1;
                                    continue;
                                }
                            };
                            pokemon.get_slot()
                        };
                        let target_str = {
                            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                                Some(p) => p,
                                None => {
                                    hit += 1;
                                    continue;
                                }
                            };
                            target_pokemon.get_slot()
                        };
                        battle.add_move(&["-anim", &pokemon_str, &active_move.name, &target_str]);
                    } else {
                        // this.battle.retargetLastMove(target);
                        battle.retarget_last_move(*target);
                    }
                }
            }
        }

        // Handle multiaccuracy (Triple Kick)
        // if (target && move.multiaccuracy && hit > 1) {
        //     let accuracy = move.accuracy;
        //     const boostTable = [1, 4 / 3, 5 / 3, 2, 7 / 3, 8 / 3, 3];
        //     if (accuracy !== true) {
        //         if (!move.ignoreAccuracy) {
        //             const boosts = this.battle.runEvent('ModifyBoost', pokemon, null, null, { ...pokemon.boosts });
        //             const boost = this.battle.clampIntRange(boosts['accuracy'], -6, 6);
        //             if (boost > 0) {
        //                 accuracy *= boostTable[boost];
        //             } else {
        //                 accuracy /= boostTable[-boost];
        //             }
        //         }
        //         if (!move.ignoreEvasion) {
        //             const boosts = this.battle.runEvent('ModifyBoost', target, null, null, { ...target.boosts });
        //             const boost = this.battle.clampIntRange(boosts['evasion'], -6, 6);
        //             if (boost > 0) {
        //                 accuracy /= boostTable[boost];
        //             } else if (boost < 0) {
        //                 accuracy *= boostTable[-boost];
        //             }
        //         }
        //     }
        //     accuracy = this.battle.runEvent('ModifyAccuracy', target, pokemon, move, accuracy);
        //     if (!move.alwaysHit) {
        //         accuracy = this.battle.runEvent('Accuracy', target, pokemon, move, accuracy);
        //         if (accuracy !== true && !this.battle.randomChance(accuracy, 100)) break;
        //     }
        // }
        if !targets_copy.is_empty() && active_move.multi_accuracy && hit > 1 {
            use crate::dex::Accuracy;

            let target = match targets_copy.first() {
                Some(SpreadMoveTarget::Target(pos)) => *pos,
                _ => {
                    // No valid target, skip multiaccuracy check
                    hit += 1;
                    continue;
                }
            };

            // let accuracy = move.accuracy;
            let mut accuracy_value = match &active_move.accuracy {
                Accuracy::AlwaysHits => {
                    // accuracy === true, skip checks
                    hit += 1;
                    continue;
                }
                Accuracy::Percent(acc) => *acc as f64,
            };

            // const boostTable = [1, 4 / 3, 5 / 3, 2, 7 / 3, 8 / 3, 3];
            let boost_table = [1.0, 4.0 / 3.0, 5.0 / 3.0, 2.0, 7.0 / 3.0, 8.0 / 3.0, 3.0];

            // if (!move.ignoreAccuracy)
            if !active_move.ignore_accuracy {
                // const boosts = this.battle.runEvent('ModifyBoost', pokemon, null, null, { ...pokemon.boosts });
                // For now, directly read pokemon boosts - full event system would modify boosts
                let accuracy_boost = {
                    let pokemon = match battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
                        Some(p) => p,
                        None => {
                            hit += 1;
                            continue;
                        }
                    };
                    pokemon.boosts.accuracy
                };

                // const boost = this.battle.clampIntRange(boosts['accuracy'], -6, 6);
                let boost = accuracy_boost.max(-6).min(6);

                // if (boost > 0) {
                //     accuracy *= boostTable[boost];
                // } else {
                //     accuracy /= boostTable[-boost];
                // }
                if boost > 0 {
                    accuracy_value *= boost_table[boost as usize];
                } else if boost < 0 {
                    accuracy_value /= boost_table[(-boost) as usize];
                }
            }

            // if (!move.ignoreEvasion)
            if !active_move.ignore_evasion {
                // const boosts = this.battle.runEvent('ModifyBoost', target, null, null, { ...target.boosts });
                let evasion_boost = {
                    let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                        Some(p) => p,
                        None => {
                            hit += 1;
                            continue;
                        }
                    };
                    target_pokemon.boosts.evasion
                };

                // const boost = this.battle.clampIntRange(boosts['evasion'], -6, 6);
                let boost = evasion_boost.max(-6).min(6);

                // if (boost > 0) {
                //     accuracy /= boostTable[boost];
                // } else if (boost < 0) {
                //     accuracy *= boostTable[-boost];
                // }
                if boost > 0 {
                    accuracy_value /= boost_table[boost as usize];
                } else if boost < 0 {
                    accuracy_value *= boost_table[(-boost) as usize];
                }
            }

            // accuracy = this.battle.runEvent('ModifyAccuracy', target, pokemon, move, accuracy);
            // TODO: Implement ModifyAccuracy event if needed

            // accuracy = this.battle.runEvent('Accuracy', target, pokemon, move, accuracy);
            // This is critical for abilities like No Guard that bypass accuracy checks
            let accuracy_result = battle.run_event(
                "Accuracy",
                Some(crate::event::EventTarget::Pokemon(target)),
                Some(attacker_pos),
                Some(&crate::battle::Effect::move_(active_move.id.clone())),
                crate::event::EventResult::Number(accuracy_value as i32),
                false,
                false
            );

            // Check result: if true, always hit; if number, use that accuracy
            let accuracy_check_passed = match accuracy_result {
                crate::event::EventResult::Boolean(true) => {
                    // No Guard or similar ability - always hit, no random check needed
                    true
                }
                crate::event::EventResult::Number(modified_acc) => {
                    // if (!move.alwaysHit) {
                    //     if (accuracy !== true && !this.battle.randomChance(accuracy, 100)) break;
                    // }
                    if active_move.always_hit {
                        true
                    } else if modified_acc == 0 {
                        // 0 in our system represents "true" (always hit)
                        true
                    } else {
                        battle.random_chance(modified_acc, 100)
                    }
                }
                _ => {
                    // Fallback: use original accuracy
                    if active_move.always_hit {
                        true
                    } else {
                        battle.random_chance(accuracy_value as i32, 100)
                    }
                }
            };

            if !accuracy_check_passed {
                // Accuracy check failed, break out of hit loop
                break;
            }
        }

        // Modifies targetsCopy (which is why it's a copy)
        // [moveDamageThisHit, targetsCopy] = this.spreadMoveHit(targetsCopy, pokemon, move, moveData);
        let (move_damage_this_hit, targets_copy_updated) = crate::battle_actions::spread_move_hit(
            battle,
            &targets_copy,
            attacker_pos,
            active_move,
            None, // hit_effect_id
            false, // is_secondary
            false, // is_self
        );
        targets_copy = targets_copy_updated;

        // if (move.smartTarget) {
        //     moveDamage.push(...moveDamageThisHit);
        // } else {
        //     moveDamage = moveDamageThisHit;
        // }
        if active_move.smart_target.unwrap_or(false) {
            move_damage.extend(move_damage_this_hit.clone());
        } else {
            move_damage = move_damage_this_hit.clone();
        }

        // if (!moveDamage.some(val => val !== false)) break;
        if !move_damage.iter().any(|d| !matches!(d, DamageResult::Failed)) {
            break;
        }

        // nullDamage = false;
        null_damage = false;

        // for (const [i, md] of moveDamage.entries()) {
        //     if (move.smartTarget && i !== hit - 1) continue;
        //     damage[i] = md === true || !md ? 0 : md;
        //     move.totalDamage += damage[i];
        // }
        for (i, md) in move_damage.iter().enumerate() {
            if active_move.smart_target.unwrap_or(false) && i != (hit as usize) - 1 {
                continue;
            }
            // Convert damage value
            let dmg = match md {
                DamageResult::Damage(d) => *d,
                _ => 0,
            };
            if i < damage.len() {
                damage[i] = DamageResult::Damage(dmg);
            }
            active_move.total_damage += dmg;
        }

        // if (move.mindBlownRecoil) {
        //     const hpBeforeRecoil = pokemon.hp;
        //     this.battle.damage(...);
        //     move.mindBlownRecoil = false;
        //     if (pokemon.hp <= pokemon.maxhp / 2 && hpBeforeRecoil > pokemon.maxhp / 2) {
        //         this.battle.runEvent('EmergencyExit', pokemon, pokemon);
        //     }
        // }
        if active_move.mindblown_recoil {
            let (hp_before_recoil, maxhp) = {
                let pokemon = match battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
                    Some(p) => p,
                    None => {
                        hit += 1;
                        continue;
                    }
                };
                (pokemon.hp, pokemon.maxhp)
            };

            let recoil_damage = (maxhp as f64 / 2.0).round() as i32;
            battle.damage(recoil_damage, Some(attacker_pos), Some(attacker_pos), Some(&Effect::move_(active_move.id.clone())), false);
            active_move.mindblown_recoil = false;
            // Also update battle.active_move since callbacks read from there, not the local active_move
            if let Some(ref mut am) = battle.active_move {
                am.mindblown_recoil = false;
            }

            let hp_after = battle.pokemon_at(attacker_pos.0, attacker_pos.1)
                .map(|p| p.hp)
                .unwrap_or(0);
            if hp_after <= maxhp / 2 && hp_before_recoil > maxhp / 2 {
                battle.run_event("EmergencyExit", Some(crate::event::EventTarget::Pokemon(attacker_pos)), Some(attacker_pos), None, EventResult::Continue, false, false);
            }
        }

        // this.battle.eachEvent('Update');
        battle.each_event("Update", None, None);

        // if (!pokemon.hp && targets.length === 1) {
        //     hit++; // report the correct number of hits for multihit moves
        //     break;
        // }
        let attacker_fainted = battle.pokemon_at(attacker_pos.0, attacker_pos.1)
            .map(|p| p.hp == 0)
            .unwrap_or(true);
        if attacker_fainted && targets.len() == 1 {
            hit += 1;
            break;
        }

        hit += 1;
    }

    // hit is 1 higher than the actual hit count
    // if (hit === 1) return damage.fill(false);
    if hit == 1 {
        return vec![DamageResult::Failed; damage.len()];
    }

    // if (nullDamage) damage.fill(false);
    if null_damage {
        damage = vec![DamageResult::Failed; damage.len()];
    }

    // this.battle.faintMessages(false, false, !pokemon.hp);
    let attacker_fainted = battle.pokemon_at(attacker_pos.0, attacker_pos.1)
        .map(|p| p.hp == 0)
        .unwrap_or(true);
    battle.faint_messages(false, false, attacker_fainted);

    // if (move.multihit && typeof move.smartTarget !== 'boolean') {
    //     this.battle.add('-hitcount', targets[0], hit - 1);
    // }
    if active_move.multi_hit.is_some() && active_move.smart_target.is_none() {
        if let Some(SpreadMoveTarget::Target(target_pos)) = targets.get(0) {
            if let Some(target_pokemon) = battle.pokemon_at(target_pos.0, target_pos.1) {
                let target_ident = format!("p{}a: {}", target_pos.0 + 1, target_pokemon.set.species);
                battle.add("-hitcount", &[
                    crate::battle::Arg::String(target_ident),
                    crate::battle::Arg::from((hit - 1).to_string()),
                ]);
            }
        }
    }

    // if ((move.recoil || move.id === 'chloroblast') && move.totalDamage) {
    //     const hpBeforeRecoil = pokemon.hp;
    //     this.battle.damage(this.calcRecoilDamage(move.totalDamage, move, pokemon), ...);
    //     if (pokemon.hp <= pokemon.maxhp / 2 && hpBeforeRecoil > pokemon.maxhp / 2) {
    //         this.battle.runEvent('EmergencyExit', pokemon, pokemon);
    //     }
    // }
    if (active_move.recoil.is_some() || active_move.id.as_str() == "chloroblast") && active_move.total_damage > 0 {
        let (hp_before_recoil, max_hp, recoil_damage, pokemon_name) = match battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
            Some(pokemon) => {
                let recoil = battle_actions::BattleActions::calc_recoil_damage(
                    active_move.total_damage,
                    active_move,
                    pokemon.maxhp,
                );
                (pokemon.hp, pokemon.maxhp, recoil, pokemon.name.clone())
            }
            None => (0, 0, 0, String::new()),
        };

        if battle.turn >= 64 && battle.turn <= 66 {
            eprintln!("[RECOIL] turn={}, pokemon={}, move={}, total_damage={}, recoil_fraction={:?}, calculated_recoil={}",
                battle.turn, pokemon_name, active_move.id.as_str(), active_move.total_damage,
                active_move.recoil, recoil_damage);
        }

        if recoil_damage > 0 {
            if battle.turn >= 64 && battle.turn <= 66 {
                eprintln!("[RECOIL] Applying {} recoil damage to {} on turn {}", recoil_damage, pokemon_name, battle.turn);
            }
            battle.damage(recoil_damage, Some(attacker_pos), Some(attacker_pos), Some(&Effect::condition("recoil")), false);

            let hp_after = battle.pokemon_at(attacker_pos.0, attacker_pos.1)
                .map(|p| p.hp)
                .unwrap_or(0);
            if hp_after <= max_hp / 2 && hp_before_recoil > max_hp / 2 {
                battle.run_event("EmergencyExit", Some(crate::event::EventTarget::Pokemon(attacker_pos)), Some(attacker_pos), None, EventResult::Continue, false, false);
            }
        }
    }

    // if (move.struggleRecoil) {
    //     const hpBeforeRecoil = pokemon.hp;
    //     let recoilDamage;
    //     if (this.dex.gen >= 5) {
    //         recoilDamage = this.battle.clampIntRange(Math.round(pokemon.baseMaxhp / 4), 1);
    //     } else {
    //         recoilDamage = this.battle.clampIntRange(this.battle.trunc(pokemon.maxhp / 4), 1);
    //     }
    //     this.battle.directDamage(recoilDamage, pokemon, pokemon, ...);
    //     if (pokemon.hp <= pokemon.maxhp / 2 && hpBeforeRecoil > pokemon.maxhp / 2) {
    //         this.battle.runEvent('EmergencyExit', pokemon, pokemon);
    //     }
    // }
    eprintln!("[STRUGGLE_RECOIL_CHECK] turn={}, move_id={}, struggle_recoil={}",
        battle.turn, active_move.id.as_str(), active_move.struggle_recoil);
    if active_move.struggle_recoil {
        let (hp_before_recoil, max_hp, base_max_hp) = match battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
            Some(pokemon) => (pokemon.hp, pokemon.maxhp, pokemon.base_maxhp),
            None => (0, 0, 0),
        };

        let recoil_damage = if battle.gen >= 5 {
            ((base_max_hp as f64 / 4.0).round() as i32).max(1)
        } else {
            ((max_hp / 4) as i32).max(1)
        };

        eprintln!("[STRUGGLE_RECOIL] turn={}, attacker={:?}, hp_before={}, max_hp={}, base_max_hp={}, recoil_damage={}",
            battle.turn, attacker_pos, hp_before_recoil, max_hp, base_max_hp, recoil_damage);

        battle.direct_damage(recoil_damage, Some(attacker_pos), Some(attacker_pos), Some(&Effect::condition("strugglerecoil")));

        let hp_after = battle.pokemon_at(attacker_pos.0, attacker_pos.1)
            .map(|p| p.hp)
            .unwrap_or(0);
        eprintln!("[STRUGGLE_RECOIL] hp_after={}", hp_after);
        if hp_after <= max_hp / 2 && hp_before_recoil > max_hp / 2 {
            battle.run_event("EmergencyExit", Some(crate::event::EventTarget::Pokemon(attacker_pos)), Some(attacker_pos), None, EventResult::Continue, false, false);
        }
    }

    // smartTarget messes up targetsCopy, but smartTarget should in theory ensure that targets will never fail, anyway
    // if (move.smartTarget) {
    //     targetsCopy = targets.slice(0);
    // }
    if active_move.smart_target.unwrap_or(false) {
        targets_copy = targets.clone();
    }

    // for (const [i, target] of targetsCopy.entries()) {
    //     if (target && pokemon !== target) {
    //         target.gotAttacked(move, moveDamage[i], pokemon);
    //         if (typeof moveDamage[i] === 'number') {
    //             target.timesAttacked += move.smartTarget ? 1 : hit - 1;
    //         }
    //     }
    // }
    for (i, target) in targets_copy.iter().enumerate() {
        if let SpreadMoveTarget::Target(target_pos) = target {
            if *target_pos != attacker_pos {
                let damage_value = move_damage.get(i).cloned();
                // Extract i32 damage from DamageResult
                let damage_int = match damage_value {
                    Some(DamageResult::Damage(d)) => d,
                    _ => 0,
                };
                if let Some(target_pokemon) = battle.pokemon_at_mut(target_pos.0, target_pos.1) {
                    target_pokemon.got_attacked(active_move, damage_int, attacker_pos.0, attacker_pos.1);
                }

                if let Some(DamageResult::Damage(_)) = damage_value {
                    let times_to_add = if active_move.smart_target.unwrap_or(false) { 1 } else { hit - 1 };
                    if let Some(target_pokemon) = battle.pokemon_at_mut(target_pos.0, target_pos.1) {
                        target_pokemon.times_attacked += times_to_add;
                    }
                }
            }
        }
    }

    // if (move.ohko && !targets[0].hp) this.battle.add('-ohko');
    if active_move.ohko.is_some() {
        if let Some(SpreadMoveTarget::Target(target_pos)) = targets.get(0) {
            let target_hp = battle.pokemon_at(target_pos.0, target_pos.1)
                .map(|p| p.hp)
                .unwrap_or(1);
            if target_hp == 0 {
                battle.add("-ohko", &[]);
            }
        }
    }

    // if (!damage.some(val => !!val || val === 0)) return damage;
    if !damage.iter().any(|d| matches!(d, DamageResult::Damage(_))) {
        return damage;
    }

    // this.battle.eachEvent('Update');
    battle.each_event("Update", None, None);

    // this.afterMoveSecondaryEvent(targetsCopy.filter(val => !!val), pokemon, move);
    let valid_targets: Vec<(usize, usize)> = targets_copy.iter()
        .filter_map(|t| match t {
            SpreadMoveTarget::Target(pos) => Some(*pos),
            _ => None,
        })
        .collect();
    // Sync battle.active_move with local active_move before calling after_move_secondary_event
    // In JS, move is passed by reference so totalDamage is immediately visible.
    // In Rust, we clone, so we need to sync before callbacks read from battle.active_move.
    if let Some(ref mut battle_active_move) = battle.active_move {
        battle_active_move.total_damage = active_move.total_damage;
    }
    crate::battle_actions::after_move_secondary_event(battle, &valid_targets, attacker_pos, active_move);

    // if (!(move.hasSheerForce && pokemon.hasAbility('sheerforce'))) {
    //     for (const [i, d] of damage.entries()) {
    //         const curDamage = targets.length === 1 ? move.totalDamage : d;
    //         if (typeof curDamage === 'number' && targets[i].hp) {
    //             const targetHPBeforeDamage = (targets[i].hurtThisTurn || 0) + curDamage;
    //             if (targets[i].hp <= targets[i].maxhp / 2 && targetHPBeforeDamage > targets[i].maxhp / 2) {
    //                 this.battle.runEvent('EmergencyExit', targets[i], pokemon);
    //             }
    //         }
    //     }
    // }
    let sheer_force_active = {
        active_move.has_sheer_force && battle.pokemon_at(attacker_pos.0, attacker_pos.1)
            .map(|p| p.has_ability(battle, &["sheerforce"]))
            .unwrap_or(false)
    };

    if !sheer_force_active {
        for (i, d) in damage.iter().enumerate() {
            let cur_damage = if targets.len() == 1 {
                active_move.total_damage
            } else {
                match d {
                    DamageResult::Damage(dmg) => *dmg,
                    _ => 0,
                }
            };

            if cur_damage > 0 {
                if let Some(SpreadMoveTarget::Target(target_pos)) = targets.get(i) {
                    let (target_hp, target_max_hp, hurt_this_turn) = {
                        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                            Some(p) => p,
                            None => continue,
                        };
                        (target.hp, target.maxhp, target.hurt_this_turn)
                    };

                    if target_hp > 0 {
                        let target_hp_before_damage = hurt_this_turn.unwrap_or(0) + cur_damage;
                        if target_hp <= target_max_hp / 2 && target_hp_before_damage > target_max_hp / 2 {
                            battle.run_event("EmergencyExit", Some(crate::event::EventTarget::Pokemon(*target_pos)), Some(attacker_pos), None, EventResult::Continue, false, false);
                        }
                    }
                }
            }
        }
    }

    // return damage;
    damage
}
