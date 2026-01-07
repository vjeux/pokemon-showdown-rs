//! BattleActions::runMoveEffects - Run move effects (boosts, healing, status, etc.)
//!
//! 1:1 port of runMoveEffects from battle-actions.ts:1201

use crate::*;
use crate::battle_actions::{SpreadMoveDamage, SpreadMoveTargets, SpreadMoveTarget, DamageResult, combine_results, SpreadMoveDamageExt};

/// Run move effects (boosts, healing, status, etc.)
/// Equivalent to runMoveEffects() in battle-actions.ts:1201
// runMoveEffects(
//     damage: SpreadMoveDamage, targets: SpreadMoveTargets, source: Pokemon,
//     move: ActiveMove, moveData: ActiveMove, isSecondary?: boolean, isSelf?: boolean
// ) {
//     let didAnything: number | boolean | null | undefined = damage.reduce(this.combineResults);
//     for (const [i, target] of targets.entries()) {
//         if (target === false) continue;
//         let hitResult;
//         let didSomething: number | boolean | null | undefined = undefined;
//
//         if (target) {
//             if (moveData.boosts && !target.fainted) {
//                 hitResult = this.battle.boost(moveData.boosts, target, source, move, isSecondary, isSelf);
//                 didSomething = this.combineResults(didSomething, hitResult);
//             }
//             if (moveData.heal && !target.fainted) {
//                 if (target.hp >= target.maxhp) {
//                     this.battle.add('-fail', target, 'heal');
//                     this.battle.attrLastMove('[still]');
//                     damage[i] = this.combineResults(damage[i], false);
//                     didAnything = this.combineResults(didAnything, null);
//                     continue;
//                 }
//                 const amount = target.baseMaxhp * moveData.heal[0] / moveData.heal[1];
//                 const d = this.battle.heal((this.battle.gen < 5 ? Math.floor : Math.round)(amount), target, source, move);
//                 if (!d && d !== 0) {
//                     if (d !== null) {
//                         this.battle.add('-fail', source);
//                         this.battle.attrLastMove('[still]');
//                     }
//                     this.battle.debug('heal interrupted');
//                     damage[i] = this.combineResults(damage[i], false);
//                     didAnything = this.combineResults(didAnything, null);
//                     continue;
//                 }
//                 didSomething = true;
//             }
//             if (moveData.status) {
//                 hitResult = target.trySetStatus(moveData.status, source, moveData.ability ? moveData.ability : move);
//                 if (!hitResult && move.status) {
//                     damage[i] = this.combineResults(damage[i], false);
//                     didAnything = this.combineResults(didAnything, null);
//                     continue;
//                 }
//                 didSomething = this.combineResults(didSomething, hitResult);
//             }
//             if (moveData.forceStatus) {
//                 hitResult = target.setStatus(moveData.forceStatus, source, move);
//                 didSomething = this.combineResults(didSomething, hitResult);
//             }
//             if (moveData.volatileStatus) {
//                 hitResult = target.addVolatile(moveData.volatileStatus, source, move);
//                 didSomething = this.combineResults(didSomething, hitResult);
//             }
//             if (moveData.sideCondition) {
//                 hitResult = target.side.addSideCondition(moveData.sideCondition, source, move);
//                 didSomething = this.combineResults(didSomething, hitResult);
//             }
//             if (moveData.slotCondition) {
//                 hitResult = target.side.addSlotCondition(target, moveData.slotCondition, source, move);
//                 didSomething = this.combineResults(didSomething, hitResult);
//             }
//             if (moveData.weather) {
//                 hitResult = this.battle.field.setWeather(moveData.weather, source, move);
//                 didSomething = this.combineResults(didSomething, hitResult);
//             }
//             if (moveData.terrain) {
//                 hitResult = this.battle.field.setTerrain(moveData.terrain, source, move);
//                 didSomething = this.combineResults(didSomething, hitResult);
//             }
//             if (moveData.pseudoWeather) {
//                 hitResult = this.battle.field.addPseudoWeather(moveData.pseudoWeather, source, move);
//                 didSomething = this.combineResults(didSomething, hitResult);
//             }
//             if (moveData.forceSwitch) {
//                 hitResult = !!this.battle.canSwitch(target.side);
//                 didSomething = this.combineResults(didSomething, hitResult);
//             }
//             // Hit events
//             //   These are like the TryHit events, except we don't need a FieldHit event.
//             //   Scroll up for the TryHit event documentation, and just ignore the "Try" part. ;)
//             if (move.target === 'all' && !isSelf) {
//                 if (moveData.onHitField) {
//                     hitResult = this.battle.singleEvent('HitField', moveData, {}, target, source, move);
//                     didSomething = this.combineResults(didSomething, hitResult);
//                 }
//             } else if ((move.target === 'foeSide' || move.target === 'allySide') && !isSelf) {
//                 if (moveData.onHitSide) {
//                     hitResult = this.battle.singleEvent('HitSide', moveData, {}, target.side, source, move);
//                     didSomething = this.combineResults(didSomething, hitResult);
//                 }
//             } else {
//                 if (moveData.onHit) {
//                     hitResult = this.battle.singleEvent('Hit', moveData, {}, target, source, move);
//                     didSomething = this.combineResults(didSomething, hitResult);
//                 }
//                 if (!isSelf && !isSecondary) {
//                     this.battle.runEvent('Hit', target, source, move);
//                 }
//             }
//         }
//         if (moveData.selfdestruct === 'ifHit' && damage[i] !== false) {
//             this.battle.faint(source, source, move);
//         }
//         if (moveData.selfSwitch) {
//             if (this.battle.canSwitch(source.side) && !source.volatiles['commanded']) {
//                 didSomething = true;
//             } else {
//                 didSomething = this.combineResults(didSomething, false);
//             }
//         }
//         // Move didn't fail because it didn't try to do anything
//         if (didSomething === undefined) didSomething = true;
//         damage[i] = this.combineResults(damage[i], didSomething === null ? false : didSomething);
//         didAnything = this.combineResults(didAnything, didSomething);
//     }
//
//     if (!didAnything && didAnything !== 0 && !moveData.self && !moveData.selfdestruct) {
//         if (!isSelf && !isSecondary) {
//             if (didAnything === false) {
//                 this.battle.add('-fail', source);
//                 this.battle.attrLastMove('[still]');
//             }
//         }
//         this.battle.debug('move failed because it did nothing');
//     } else if (move.selfSwitch && source.hp && !source.volatiles['commanded']) {
//         source.switchFlag = move.id;
//     }
//
//     return damage;
// }
pub fn run_move_effects(
    battle: &mut Battle,
    mut damages: SpreadMoveDamage,
    targets: &SpreadMoveTargets,
    source_pos: (usize, usize),
    active_move: &crate::battle_actions::ActiveMove,
    move_data: &crate::battle_actions::ActiveMove,
    is_secondary: bool,
    is_self: bool,
) -> SpreadMoveDamage {
    // JavaScript signature: runMoveEffects(damage, targets, source, move, moveData, isSecondary?, isSelf?)
    // Both move and moveData are ActiveMove types in JavaScript
    // In many cases they're the same, but moveData can differ for secondary effects

    // let didAnything: number | boolean | null | undefined = damage.reduce(this.combineResults);
    let mut did_anything = damages.reduce();

    // for (const [i, target] of targets.entries()) {
    for (i, target) in targets.iter().enumerate() {
        // if (target === false) continue;
        if matches!(target, SpreadMoveTarget::Failed) {
            continue;
        }

        let mut did_something = DamageResult::Undefined;

        // if (target) {
        if matches!(target, SpreadMoveTarget::Target(_)) {
            let target_pos = match target {
                SpreadMoveTarget::Target(pos) => *pos,
                _ => continue,
            };

            // if (moveData.boosts && !target.fainted) {
            //     hitResult = this.battle.boost(moveData.boosts, target, source, move, isSecondary, isSelf);
            //     didSomething = this.combineResults(didSomething, hitResult);
            // }
            if let Some(ref boosts) = move_data.boosts {
                // Check !target.fainted
                let target_fainted = {
                    let pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                        Some(p) => p,
                        None => continue,
                    };
                    pokemon.fainted
                };

                if !target_fainted {
                    // Convert BoostsTable to Vec<(&str, i8)> - only include non-zero boosts
                    let mut boosts_vec: Vec<(&str, i8)> = Vec::new();
                    if boosts.atk != 0 { boosts_vec.push(("atk", boosts.atk)); }
                    if boosts.def != 0 { boosts_vec.push(("def", boosts.def)); }
                    if boosts.spa != 0 { boosts_vec.push(("spa", boosts.spa)); }
                    if boosts.spd != 0 { boosts_vec.push(("spd", boosts.spd)); }
                    if boosts.spe != 0 { boosts_vec.push(("spe", boosts.spe)); }
                    if boosts.accuracy != 0 { boosts_vec.push(("accuracy", boosts.accuracy)); }
                    if boosts.evasion != 0 { boosts_vec.push(("evasion", boosts.evasion)); }

                    let hit_result = battle.boost(
                        &boosts_vec,
                        target_pos,
                        Some(source_pos),
                        Some(active_move.id.as_str()),
                        is_secondary,
                        is_self,
                    );

                    // Convert bool to DamageResult
                    let hit_result_dr = if hit_result {
                        DamageResult::Success
                    } else {
                        DamageResult::Failed
                    };
                    did_something = combine_results(did_something, hit_result_dr);
                }
            }

            // if (moveData.heal && !target.fainted) {
            if let Some(heal_fraction) = move_data.heal {
                // Check !target.fainted and get hp/maxhp
                let (target_fainted, target_hp, target_maxhp, target_base_maxhp) = {
                    let pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                        Some(p) => p,
                        None => continue,
                    };
                    (pokemon.fainted, pokemon.hp, pokemon.maxhp, pokemon.base_maxhp)
                };

                if !target_fainted {
                    // if (target.hp >= target.maxhp) {
                    if target_hp >= target_maxhp {
                        //     this.battle.add('-fail', target, 'heal');
                        //     this.battle.attrLastMove('[still]');
                        let target_ident = {
                            let pokemon = battle.pokemon_at(target_pos.0, target_pos.1).unwrap();
                            format!("p{}a: {}", target_pos.0 + 1, pokemon.set.species)
                        };
                        battle.add("|-fail|", &[
                            crate::battle::Arg::String(target_ident),
                            crate::battle::Arg::String("heal".to_string()),
                        ]);
                        battle.attr_last_move(&["[still]"]);
                        //     damage[i] = this.combineResults(damage[i], false);
                        damages[i] = combine_results(damages[i], DamageResult::Failed);
                        //     didAnything = this.combineResults(didAnything, null);
                        // Note: JavaScript null is tricky - setting to Undefined
                        did_anything = combine_results(did_anything, DamageResult::Undefined);
                        //     continue;
                        continue;
                    }
                    // const amount = target.baseMaxhp * moveData.heal[0] / moveData.heal[1];
                    let amount = target_base_maxhp * heal_fraction.0 / heal_fraction.1;
                    // const d = this.battle.heal((this.battle.gen < 5 ? Math.floor : Math.round)(amount), target, source, move);
                    let amount_rounded = if battle.gen < 5 {
                        amount // floor (already an integer after division)
                    } else {
                        // Round: add 0.5 before truncating. In Rust integer division truncates.
                        // For proper rounding with integer math: (numerator + denominator/2) / denominator
                        (target_base_maxhp * heal_fraction.0 + heal_fraction.1 / 2) / heal_fraction.1
                    };
                    let d = battle.heal(
                        amount_rounded,
                        Some(target_pos),
                        Some(source_pos),
                        Some(&active_move.id),
                    );
                    // if (!d && d !== 0) {
                    match d {
                        None | Some(0) if d != Some(0) => {
                            //     if (d !== null) {
                            if d.is_some() {
                                //         this.battle.add('-fail', source);
                                let source_ident = {
                                    let pokemon = battle.pokemon_at(source_pos.0, source_pos.1).unwrap();
                                    format!("p{}a: {}", source_pos.0 + 1, pokemon.set.species)
                                };
                                battle.add("|-fail|", &[crate::battle::Arg::String(source_ident)]);
                                //         this.battle.attrLastMove('[still]');
                                battle.attr_last_move(&["[still]"]);
                            }
                            //     this.battle.debug('heal interrupted');
                            battle.debug("heal interrupted");
                            //     damage[i] = this.combineResults(damage[i], false);
                            damages[i] = combine_results(damages[i], DamageResult::Failed);
                            //     didAnything = this.combineResults(didAnything, null);
                            did_anything = combine_results(did_anything, DamageResult::Undefined);
                            //     continue;
                            continue;
                        }
                        _ => {}
                    }
                    //     didSomething = true;
                    did_something = DamageResult::Success;
                }
            }

            // if (moveData.status) {
            if let Some(ref status) = move_data.status {
                //     hitResult = target.trySetStatus(moveData.status, source, moveData.ability ? moveData.ability : move);
                let status_id = ID::new(status);
                let hit_result = Pokemon::try_set_status(
                    battle,
                    target_pos,
                    status_id,
                    None, // source_effect - using active_move effect instead
                );
                //     if (!hitResult && move.status) {
                // Check if active_move also has status (primary status move)
                let move_has_status = active_move.status.is_some();
                if !hit_result && move_has_status {
                    //         damage[i] = this.combineResults(damage[i], false);
                    damages[i] = combine_results(damages[i], DamageResult::Failed);
                    //         didAnything = this.combineResults(didAnything, null);
                    did_anything = combine_results(did_anything, DamageResult::Undefined);
                    //         continue;
                    continue;
                }
                //     didSomething = this.combineResults(didSomething, hitResult);
                let hit_result_dr = if hit_result {
                    DamageResult::Success
                } else {
                    DamageResult::Failed
                };
                did_something = combine_results(did_something, hit_result_dr);
            }

            // if (moveData.forceStatus) {
            // TODO: ActiveMove doesn't have forceStatus field yet
            // Need to add this field to ActiveMove struct

            // if (moveData.volatileStatus) {
            if let Some(ref volatile_status) = move_data.volatile_status {
                //     hitResult = target.addVolatile(moveData.volatileStatus, source, move);
                let volatile_id = ID::new(volatile_status);
                let hit_result = Pokemon::add_volatile(
                    battle,
                    target_pos,
                    volatile_id,
                    Some(source_pos),
                    Some(&active_move.id),
                    None, // linked_status
                    None, // embedded_condition
                );
                //     didSomething = this.combineResults(didSomething, hitResult);
                let hit_result_dr = if hit_result {
                    DamageResult::Success
                } else {
                    DamageResult::Failed
                };
                did_something = combine_results(did_something, hit_result_dr);
            }

            // if (moveData.sideCondition) {
            eprintln!("[SIDE_CONDITION] move_id={}, has side_condition={}", active_move.id.as_str(), move_data.side_condition.is_some());
            if let Some(ref side_condition) = move_data.side_condition {
                eprintln!("[SIDE_CONDITION] Applying side condition '{}' to side {}", side_condition, target_pos.0);
                //     hitResult = target.side.addSideCondition(moveData.sideCondition, source, move);
                let condition_id = ID::new(side_condition);
                let hit_result = battle.add_side_condition(
                    target_pos.0,
                    condition_id,
                    Some(source_pos),
                    Some(&active_move.id),
                );
                eprintln!("[SIDE_CONDITION] add_side_condition returned: {}", hit_result);
                //     didSomething = this.combineResults(didSomething, hitResult);
                let hit_result_dr = if hit_result {
                    DamageResult::Success
                } else {
                    DamageResult::Failed
                };
                did_something = combine_results(did_something, hit_result_dr);
            }

            // INFRASTRUCTURE FIX: Handle moves with embedded conditions that have onSideStart
            // E.g., gmaxvolcalith has a "condition" with onSideStart, onResidual, onSideEnd
            // In this case, the move itself becomes a side condition
            if move_data.side_condition.is_none() {
                // Look up the move data from dex to check for embedded condition with onSideStart
                if let Some(original_move_data) = battle.dex.moves().get(active_move.id.as_str()) {
                    // Check if move has embedded condition with onSideStart
                    if let Some(ref condition_data) = original_move_data.condition {
                        if condition_data.extra.get("onSideStart").is_some() {
                            eprintln!("[SIDE_CONDITION_EMBEDDED] move_id={} has embedded condition with onSideStart, applying as side condition to side {}",
                                active_move.id.as_str(), target_pos.0);
                            // Apply the move itself as a side condition (use move ID, not condition ID)
                            let side_condition_id = active_move.id.clone();
                            let hit_result = battle.add_side_condition(
                                target_pos.0,
                                side_condition_id,
                                Some(source_pos),
                                Some(&active_move.id),
                            );
                            eprintln!("[SIDE_CONDITION_EMBEDDED] add_side_condition returned: {}", hit_result);
                            let hit_result_dr = if hit_result {
                                DamageResult::Success
                            } else {
                                DamageResult::Failed
                            };
                            did_something = combine_results(did_something, hit_result_dr);
                        }
                    }
                }
            }

            // if (moveData.slotCondition) {
            if let Some(ref slot_condition) = move_data.slot_condition {
                //     hitResult = target.side.addSlotCondition(target, moveData.slotCondition, source, move);
                let condition_id = ID::new(slot_condition);
                let hit_result = {
                    let side = &mut battle.sides[target_pos.0];
                    side.add_slot_condition(target_pos.1, condition_id, None) // slot, condition_id, duration
                };
                //     didSomething = this.combineResults(didSomething, hitResult);
                let hit_result_dr = if hit_result {
                    DamageResult::Success
                } else {
                    DamageResult::Failed
                };
                did_something = combine_results(did_something, hit_result_dr);
            }

            // if (moveData.weather) {
            if let Some(ref weather) = move_data.weather {
                //     hitResult = this.battle.field.setWeather(moveData.weather, source, move);
                let weather_id = ID::new(weather);
                let hit_result = battle.set_weather(
                    weather_id,
                    Some(source_pos),
                    Some(active_move.id.clone()),
                );
                //     didSomething = this.combineResults(didSomething, hitResult);
                // JavaScript returns true/false/null, convert to DamageResult
                let hit_result_dr = match hit_result {
                    Some(true) => DamageResult::Success,
                    Some(false) => DamageResult::Failed,
                    None => DamageResult::Undefined,
                };
                did_something = combine_results(did_something, hit_result_dr);
            }

            // if (moveData.terrain) {
            if let Some(ref terrain) = move_data.terrain {
                //     hitResult = this.battle.field.setTerrain(moveData.terrain, source, move);
                let terrain_id = ID::new(terrain);
                // TODO: field.set_terrain should be a Battle-level method that handles events
                // For now, using the simple field implementation
                let hit_result = battle.set_terrain(terrain_id, None); // duration
                //     didSomething = this.combineResults(didSomething, hitResult);
                let hit_result_dr = if hit_result {
                    DamageResult::Success
                } else {
                    DamageResult::Failed
                };
                did_something = combine_results(did_something, hit_result_dr);
            }

            // if (moveData.pseudoWeather) {
            if let Some(ref pseudo_weather) = move_data.pseudo_weather {
                //     hitResult = this.battle.field.addPseudoWeather(moveData.pseudoWeather, source, move);
                let pseudo_weather_id = ID::new(pseudo_weather);
                // TODO: field.add_pseudo_weather should be a Battle-level method that handles events
                // For now, using the simple field implementation
                let hit_result = battle.add_pseudo_weather(pseudo_weather_id, None); // duration
                //     didSomething = this.combineResults(didSomething, hitResult);
                let hit_result_dr = if hit_result {
                    DamageResult::Success
                } else {
                    DamageResult::Failed
                };
                did_something = combine_results(did_something, hit_result_dr);
            }

            // if (moveData.forceSwitch) {
            if move_data.force_switch {
                //     hitResult = !!this.battle.canSwitch(target.side);
                let can_switch_count = battle.can_switch(target_pos.0);
                let hit_result = can_switch_count > 0; // !! converts to boolean
                //     didSomething = this.combineResults(didSomething, hitResult);
                let hit_result_dr = if hit_result {
                    DamageResult::Success
                } else {
                    DamageResult::Failed
                };
                did_something = combine_results(did_something, hit_result_dr);
            }

            // Hit events - onHitField, onHitSide, onHit
            //   These are like the TryHit events, except we don't need a FieldHit event.
            //   Scroll up for the TryHit event documentation, and just ignore the "Try" part. ;)

            // if (move.target === 'all' && !isSelf) {
            if active_move.target == "all" && !is_self {
                //     if (moveData.onHitField) {
                if battle.has_callback(&move_data.id, "HitField") {
                    //         hitResult = this.battle.singleEvent('HitField', moveData, {}, target, source, move);
                    let hit_result = battle.single_event(
                        "HitField",
                        &move_data.id,
                        Some(target_pos),
                        Some(source_pos),
                        Some(&active_move.id),
                        None,
                    );
                    //         didSomething = this.combineResults(didSomething, hitResult);
                    did_something = combine_results(did_something, hit_result.into());
                }
            }
            // } else if ((move.target === 'foeSide' || move.target === 'allySide') && !isSelf) {
            else if (active_move.target == "foeSide" || active_move.target == "allySide") && !is_self {
                //     if (moveData.onHitSide) {
                if battle.has_callback(&move_data.id, "HitSide") {
                    //         hitResult = this.battle.singleEvent('HitSide', moveData, {}, target.side, source, move);
                    let hit_result = battle.single_event(
                        "HitSide",
                        &move_data.id,
                        Some(target_pos),
                        Some(source_pos),
                        Some(&active_move.id),
                        None,
                    );
                    //         didSomething = this.combineResults(didSomething, hitResult);
                    did_something = combine_results(did_something, hit_result.into());
                }
            } else {
                //     if (moveData.onHit) {
                if battle.has_callback(&move_data.id, "Hit") {
                    //         hitResult = this.battle.singleEvent('Hit', moveData, {}, target, source, move);
                    let hit_result = battle.single_event(
                        "Hit",
                        &move_data.id,
                        Some(target_pos),
                        Some(source_pos),
                        Some(&active_move.id),
                        None,
                    );
                    //         didSomething = this.combineResults(didSomething, hitResult);
                    did_something = combine_results(did_something, hit_result.into());
                }
                //     if (!isSelf && !isSecondary) {
                if !is_self && !is_secondary {
                    //         this.battle.runEvent('Hit', target, source, move);
                    battle.run_event(
                "Hit",
                Some(crate::event::EventTarget::Pokemon(target_pos)),
                        Some(source_pos),
                        Some(&active_move.id),
                        crate::event::EventResult::Continue,
                        false,
                        false,
                    );

                    // Call onHit callbacks for each of the target's volatile conditions
                    // In JavaScript, this happens automatically through runEvent's effect iteration
                    // In Rust, we need to explicitly dispatch to volatile condition callbacks
                    // AND set up current_effect_state to point to each volatile's EffectState
                    let target_volatile_ids: Vec<String> = {
                        match battle.pokemon_at(target_pos.0, target_pos.1) {
                            Some(target_pokemon) => {
                                target_pokemon.volatiles.keys()
                                    .map(|id| id.as_str().to_string())
                                    .collect()
                            }
                            None => vec![],
                        }
                    };

                    for volatile_id in target_volatile_ids {
                        // Set up current_effect_state with a clone of this volatile's EffectState
                        // JavaScript does this automatically in runEvent before calling each callback
                        {
                            let target_pokemon = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
                                Some(p) => p,
                                None => continue,
                            };

                            let volatile_id_obj = crate::dex_data::ID::from(volatile_id.as_str());
                            if let Some(volatile_state) = target_pokemon.volatiles.get(&volatile_id_obj) {
                                // Clone the state and set as current_effect_state
                                battle.current_effect_state = Some(volatile_state.clone());
                            } else {
                                continue;
                            }
                        }

                        // Call the volatile's onHit callback if it has one
                        // Pass target_pos as the Pokemon with the volatile, and source_pos as the attacker
                        crate::data::move_callbacks::dispatch_condition_on_hit(
                            battle,
                            &volatile_id,
                            target_pos,  // Pokemon with the volatile
                            source_pos,  // Pokemon using the move
                        );

                        // Write the modified state back to the volatile
                        if let Some(modified_state) = battle.current_effect_state.take() {
                            let target_pokemon = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
                                Some(p) => p,
                                None => continue,
                            };

                            let volatile_id_obj = crate::dex_data::ID::from(volatile_id.as_str());
                            target_pokemon.volatiles.insert(volatile_id_obj, modified_state);
                        }
                    }
                }
            }
        }

        // if (moveData.selfdestruct === 'ifHit' && damage[i] !== false) {
        if let Some(ref selfdestruct) = move_data.self_destruct {
            if selfdestruct == "ifHit" && !matches!(damages[i], DamageResult::Failed) {
                //     this.battle.faint(source, source, move);
                battle.faint(source_pos, Some(source_pos), Some(active_move.id.as_str()));
            }
        }

        // if (moveData.selfSwitch) {
        if move_data.self_switch.is_some() {
            //     if (this.battle.canSwitch(source.side) && !source.volatiles['commanded']) {
            let can_switch_count = battle.can_switch(source_pos.0);
            let has_commanded = {
                match battle.pokemon_at(source_pos.0, source_pos.1) {
                    Some(p) => p.volatiles.contains_key(&ID::new("commanded")),
                    None => false,
                }
            };

            if can_switch_count > 0 && !has_commanded {
                //         didSomething = true;
                did_something = DamageResult::Success;
            } else {
                //         didSomething = this.combineResults(didSomething, false);
                did_something = combine_results(did_something, DamageResult::Failed);
            }
        }

        // Move didn't fail because it didn't try to do anything
        // if (didSomething === undefined) didSomething = true;
        if matches!(did_something, DamageResult::Undefined) {
            did_something = DamageResult::Success;
        }

        // damage[i] = this.combineResults(damage[i], didSomething === null ? false : didSomething);
        // Note: JavaScript null doesn't have a direct DamageResult equivalent
        // Assuming "null" would be represented as Undefined in our context
        damages[i] = combine_results(damages[i], did_something);

        // didAnything = this.combineResults(didAnything, didSomething);
        did_anything = combine_results(did_anything, did_something);
    }

    // if (!didAnything && didAnything !== 0 && !moveData.self && !moveData.selfdestruct) {
    let did_anything_is_zero = matches!(did_anything, DamageResult::Damage(0) | DamageResult::HitSubstitute);
    let did_anything_is_falsy = matches!(did_anything, DamageResult::Failed | DamageResult::Undefined);

    if did_anything_is_falsy && !did_anything_is_zero
        && move_data.self_effect.is_none()
        && move_data.self_destruct.is_none() {

        if !is_self && !is_secondary {
            // if (didAnything === false) {
            if matches!(did_anything, DamageResult::Failed) {
                //     this.battle.add('-fail', source);
                let source_ident = {
                    let pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
                        Some(p) => p,
                        None => return damages,
                    };
                    format!("p{}a: {}", source_pos.0 + 1, pokemon.set.species)
                };
                battle.add("|-fail|", &[crate::battle::Arg::String(source_ident)]);
                //     this.battle.attrLastMove('[still]');
                battle.attr_last_move(&["[still]"]);
            }
        }
        //     this.battle.debug('move failed because it did nothing');
        battle.debug("move failed because it did nothing");
    }
    // else if (move.selfSwitch && source.hp && !source.volatiles['commanded']) {
    else if active_move.self_switch.is_some() {
        //     source.switchFlag = move.id;
        // Check source.hp and source.volatiles['commanded']
        let (source_hp, has_commanded) = {
            let pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
                Some(p) => p,
                None => return damages,
            };
            (pokemon.hp, pokemon.volatiles.contains_key(&ID::new("commanded")))
        };

        if source_hp > 0 && !has_commanded {
            // Set switch flag
            let pokemon = match battle.pokemon_at_mut(source_pos.0, source_pos.1) {
                Some(p) => p,
                None => return damages,
            };
            pokemon.switch_flag = Some(active_move.id.to_string());
        }
    }

    // return damage;
    damages
}
