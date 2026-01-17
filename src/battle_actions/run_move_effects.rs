//! BattleActions::runMoveEffects - Run move effects (boosts, healing, status, etc.)
//!
//! 1:1 port of runMoveEffects from battle-actions.ts:1201

use crate::*;
use crate::battle_actions::{SpreadMoveDamage, SpreadMoveTargets, SpreadMoveTarget, DamageResult, combine_results, SpreadMoveDamageExt, HitEffect};
use crate::battle::Effect;

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

pub fn run_move_effects<'a>(
    battle: &mut Battle,
    mut damages: SpreadMoveDamage,
    targets: &SpreadMoveTargets,
    source_pos: (usize, usize),
    active_move: &crate::battle_actions::ActiveMove,
    hit_effect: HitEffect<'a>,
    is_secondary: bool,
    is_self: bool,
) -> SpreadMoveDamage {
    // JavaScript signature: runMoveEffects(damage, targets, source, move, moveData, isSecondary?, isSelf?)
    // Both move and moveData are ActiveMove types in JavaScript
    // In many cases they're the same, but moveData can differ for secondary effects
    // hit_effect provides access to the moveData fields via accessor methods

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
            if let Some(boosts) = hit_effect.boosts() {
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
            if let Some(heal_fraction) = hit_effect.heal() {
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
                        Some(&crate::battle::Effect::move_(active_move.id.clone())),
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
            // IMPORTANT: Check active_move.status FIRST (can be set dynamically by moves like Psycho Shift)
            // If not set, fall back to hit_effect.status() (static move data)
            //
            // EXCEPTION: For self effects (is_self=true), we should only use hit_effect.status()
            // because moveData.self.status is separate from the dynamically-set move.status.
            // The dynamically-set status (like from Psycho Shift's onTryHit) applies to the target,
            // not to the source via self effects.
            let status_to_apply = if is_self {
                // For self effects, only use moveData.self.status (from hit_effect)
                hit_effect.status().map(|s| s.as_str())
            } else if let Some(ref dynamic_status) = active_move.status {
                Some(dynamic_status.as_str())
            } else {
                hit_effect.status().map(|s| s.as_str())
            };

            if let Some(status) = status_to_apply {
                //     hitResult = target.trySetStatus(moveData.status, source, moveData.ability ? moveData.ability : move);
                let status_id = ID::new(status);
                // JavaScript: moveData.ability ? moveData.ability : move
                // If the move has an ability (like Rest's Insomnia check), use that as the source effect
                // Otherwise use the move itself as the source effect
                let source_effect = if let Some(ref ability_id) = active_move.ability {
                    crate::battle::Effect::ability(ability_id.clone())
                } else {
                    crate::battle::Effect::move_(active_move.id.clone())
                };
                let hit_result = Pokemon::try_set_status(
                    battle,
                    target_pos,
                    status_id,
                    Some(source_pos), // source
                    Some(&source_effect), // source_effect - the move or ability causing the status
                );
                //     if (!hitResult && move.status) {
                // Check if the move is a primary status move (status was set)
                let move_has_status = active_move.status.is_some() || hit_effect.status().is_some();
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
            if let Some(volatile_status) = hit_effect.volatile_status() {
                //     hitResult = target.addVolatile(moveData.volatileStatus, source, move);
                let volatile_id = ID::new(volatile_status);

                // Look up the move's embedded condition from dex
                // Moves like bide have `condition: { duration: 3, ... }` embedded in them
                // Clone to avoid borrow conflict
                let embedded_condition = battle.dex.moves().get_by_id(&active_move.id)
                    .and_then(|m| m.condition.clone());

                let hit_result = Pokemon::add_volatile(
                    battle,
                    target_pos,
                    volatile_id,
                    Some(source_pos),
                    Some(&crate::battle::Effect::move_(active_move.id.clone())),
                    None, // linked_status
                    embedded_condition.as_ref(), // embedded_condition from move's condition field
                );
                //     didSomething = this.combineResults(didSomething, hitResult);
                // add_volatile returns Option<bool>:
                // - Some(true): success → DamageResult::Success
                // - Some(false): failure → DamageResult::Failed
                // - None: restart case → DamageResult::Undefined (matches JS undefined)
                let hit_result_dr = match hit_result {
                    Some(true) => DamageResult::Success,
                    Some(false) => DamageResult::Failed,
                    None => DamageResult::Undefined,
                };
                did_something = combine_results(did_something, hit_result_dr);
            }

            // if (moveData.sideCondition) {
            if let Some(side_condition) = hit_effect.side_condition() {
                //     hitResult = target.side.addSideCondition(moveData.sideCondition, source, move);
                let condition_id = ID::new(side_condition);
                let move_effect = Effect::move_(active_move.id.clone());
                let hit_result = battle.add_side_condition(
                    target_pos.0,
                    condition_id,
                    Some(source_pos),
                    Some(&move_effect),
                );
                //     didSomething = this.combineResults(didSomething, hitResult);
                let hit_result_dr = if hit_result {
                    DamageResult::Success
                } else {
                    DamageResult::Failed
                };
                did_something = combine_results(did_something, hit_result_dr);
            }

            // NOTE: G-Max moves with embedded conditions (like gmaxcannonade, gmaxvolcalith)
            // use self.onHit callbacks to add side conditions to foe sides. Do NOT add
            // side conditions here based on condition.onSideStart, as that would incorrectly
            // apply them to each target's side instead of foe sides only.

            // if (moveData.slotCondition) {
            if let Some(slot_condition) = hit_effect.slot_condition() {
                //     hitResult = target.side.addSlotCondition(target, moveData.slotCondition, source, move);
                let condition_id = ID::new(slot_condition);
                let source_effect = Effect::move_(active_move.id.clone());
                let hit_result = battle.add_slot_condition(
                    target_pos,
                    condition_id,
                    Some(source_pos),
                    Some(&source_effect),
                    None,
                );
                //     didSomething = this.combineResults(didSomething, hitResult);
                let hit_result_dr = if hit_result {
                    DamageResult::Success
                } else {
                    DamageResult::Failed
                };
                did_something = combine_results(did_something, hit_result_dr);
            }

            // if (moveData.weather) {
            if let Some(weather) = hit_effect.weather() {
                //     hitResult = this.battle.field.setWeather(moveData.weather, source, move);
                let weather_id = ID::new(weather);
                let hit_result = battle.set_weather(
                    weather_id,
                    Some(source_pos),
                    Some(Effect::move_(active_move.id.clone())),
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
            if let Some(terrain) = hit_effect.terrain() {
                //     hitResult = this.battle.field.setTerrain(moveData.terrain, source, move);
                let terrain_id = ID::new(terrain);
                let terrain_effect = Some(Effect::move_(active_move.id.clone()));
                let hit_result = battle.set_terrain(terrain_id, Some(source_pos), terrain_effect);
                //     didSomething = this.combineResults(didSomething, hitResult);
                let hit_result_dr = if hit_result {
                    DamageResult::Success
                } else {
                    DamageResult::Failed
                };
                did_something = combine_results(did_something, hit_result_dr);
            }

            // if (moveData.pseudoWeather) {
            if let Some(pseudo_weather) = hit_effect.pseudo_weather() {
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
            if hit_effect.force_switch() {
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

            // Get effect_id for callbacks (use active_move.id if hit_effect is a SecondaryEffect)
            let effect_id = hit_effect.id().unwrap_or(&active_move.id);

            // if (move.target === 'all' && !isSelf) {
            if active_move.target == "all" && !is_self {
                //     if (moveData.onHitField) {
                if battle.has_move_id_callback(effect_id, "HitField") {
                    //         hitResult = this.battle.singleEvent('HitField', moveData, {}, target, source, move);
                    let hit_result = battle.single_event(
                        "HitField",
                        &crate::battle::Effect::move_(effect_id.clone()),
                        None,
                        Some(target_pos),
                        Some(source_pos),
                        Some(&Effect::move_(active_move.id.clone())),
                        None,
                    );
                    //         didSomething = this.combineResults(didSomething, hitResult);
                    did_something = combine_results(did_something, hit_result.into());
                }
            }
            // } else if ((move.target === 'foeSide' || move.target === 'allySide') && !isSelf) {
            else if (active_move.target == "foeSide" || active_move.target == "allySide") && !is_self {
                //     if (moveData.onHitSide) {
                if battle.has_move_id_callback(effect_id, "HitSide") {
                    //         hitResult = this.battle.singleEvent('HitSide', moveData, {}, target.side, source, move);
                    let hit_result = battle.single_event(
                        "HitSide",
                        &crate::battle::Effect::move_(effect_id.clone()),
                        None,
                        Some(target_pos),
                        Some(source_pos),
                        Some(&Effect::move_(active_move.id.clone())),
                        None,
                    );
                    //         didSomething = this.combineResults(didSomething, hitResult);
                    did_something = combine_results(did_something, hit_result.into());
                }
            } else {
                //     if (moveData.onHit) {
                // For secondary effects, the onHit callback is defined in the secondary object,
                // not directly on the move. We need to check if the hit_effect is a Secondary
                // and if THIS SPECIFIC secondary has an onHit callback (has_on_hit flag).
                //
                // IMPORTANT: Only the move's original secondaries have has_on_hit=true.
                // Item-added secondaries (like King's Rock flinch) have has_on_hit=false.
                // This prevents calling the move's secondary.onHit for item-added secondaries.
                //
                // IMPORTANT: For self effects (is_self=true), we need to check for self.onHit
                // callbacks and dispatch to them directly. The normal has_callback check
                // doesn't find self.onHit callbacks.
                let has_hit_callback = match &hit_effect {
                    HitEffect::Secondary(secondary) => {
                        // For secondaries, check if THIS specific secondary has an onHit callback
                        // This distinguishes move's original secondaries from item-added ones
                        secondary.has_on_hit
                    },
                    _ => {
                        if is_self {
                            // For self effects, check for self.onHit callbacks
                            battle.move_has_self_callback(active_move.id.as_str(), "Hit")
                        } else {
                            // Check both dex data AND dispatch registry (for dynamically set callbacks like Fling)
                            battle.has_move_id_callback(effect_id, "Hit") ||
                            crate::data::move_callbacks::has_on_hit(effect_id.as_str())
                        }
                    },
                };

                if has_hit_callback {
                    if is_self {
                        // JavaScript: singleEvent('Hit', moveData.self, {}, source, source, move)
                        // For self effects, target is source (the move user)
                        // We use Effect::move_self() to indicate this is moveData.self
                        let hit_result = battle.single_event(
                            "Hit",
                            &crate::battle::Effect::move_self(effect_id.clone()),  // moveData.self
                            None,
                            Some(source_pos),  // target is source for self effects
                            Some(source_pos),  // source is also source
                            Some(&Effect::move_(active_move.id.clone())),  // sourceEffect is the move
                            None,
                        );
                        did_something = combine_results(did_something, hit_result.into());
                    } else {
                        //         hitResult = this.battle.singleEvent('Hit', moveData, {}, target, source, move);
                        let hit_result = battle.single_event(
                            "Hit",
                            &crate::battle::Effect::move_(effect_id.clone()),
                            None,
                            Some(target_pos),
                            Some(source_pos),
                            Some(&Effect::move_(active_move.id.clone())),
                            None,
                        );
                        //         didSomething = this.combineResults(didSomething, hitResult);
                        did_something = combine_results(did_something, hit_result.into());
                    }
                }
                //     if (!isSelf && !isSecondary) {
                if !is_self && !is_secondary {
                    //         this.battle.runEvent('Hit', target, source, move);
                    battle.run_event(
                        "Hit",
                        Some(crate::event::EventTarget::Pokemon(target_pos)),
                        Some(source_pos),
                        Some(&crate::battle::Effect::move_(active_move.id.clone())),
                        crate::event::EventResult::Continue,
                        false,
                        false,
                    );
                }
            }
        }

        // if (moveData.selfdestruct === 'ifHit' && damage[i] !== false) {
        if let Some(selfdestruct) = hit_effect.self_destruct() {
            if selfdestruct == "ifHit" && !matches!(damages[i], DamageResult::Failed) {
                //     this.battle.faint(source, source, move);
                battle.faint(source_pos, Some(source_pos), Some(active_move.id.as_str()));
            }
        }

        // if (moveData.selfSwitch) {
        // IMPORTANT: Use battle.active_move, not hit_effect!
        // The on_hit callback (e.g., partingshot) can delete move.selfSwitch by setting
        // battle.active_move.self_switch = None. We need to check the updated value.
        if battle.active_move.as_ref().map(|m| m.self_switch.is_some()).unwrap_or(false) {
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
        && hit_effect.self_effect_data().is_none()
        && hit_effect.self_destruct().is_none() {

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
    // IMPORTANT: Use battle.active_move, not the local active_move parameter!
    // The on_hit callback (e.g., partingshot) can delete move.selfSwitch by setting
    // battle.active_move.self_switch = None. We need to check the updated value.
    else if battle.active_move.as_ref().map(|m| m.self_switch.is_some()).unwrap_or(false) {
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
