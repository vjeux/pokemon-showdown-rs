//! BattleActions::hitStepMoveHitLoop - Main loop for multi-hit moves
//!
//! 1:1 port of hitStepMoveHitLoop from battle-actions.ts

use crate::*;
use crate::battle_actions::{SpreadMoveDamage, SpreadMoveDamageValue, SpreadMoveTargets, SpreadMoveTarget, ActiveMove};

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
    let mut damage: SpreadMoveDamage = vec![SpreadMoveDamageValue::Damage(0); targets.len()];

    // move.totalDamage = 0;
    active_move.total_damage = 0;

    // pokemon.lastDamage = 0;
    if let Some(pokemon) = battle.pokemon_at_mut(attacker_pos.0, attacker_pos.1) {
        pokemon.last_damage = 0;
    }

    // let targetHits = move.multihit || 1;
    let mut target_hits = active_move.multi_hit.unwrap_or(1);

    // if (Array.isArray(targetHits)) {
    // Note: In Rust, multi_hit is Option<i32>, so we can't represent arrays
    // This would need to be handled differently - for now we'll skip this logic
    // TODO: Implement array-based multihit (would require changing ActiveMove.multi_hit type)

    // Special case for loaded dice with targetHits == 10
    // if (targetHits === 10 && pokemon.hasItem('loadeddice')) targetHits -= this.battle.random(7);
    if target_hits == 10 {
        let has_loaded_dice = battle.pokemon_at(attacker_pos.0, attacker_pos.1)
            .map(|p| p.has_item(battle, "loadeddice"))
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
    let is_sleep_usable = active_move.sleep_usable; // TODO: Check sourceEffect move's sleepUsable

    // let targetsCopy: (Pokemon | false | null)[] = targets.slice(0);
    let mut targets_copy = targets.clone();

    // let hit: number;
    // for (hit = 1; hit <= targetHits; hit++) {
    let mut hit = 1;
    while hit <= target_hits {
        // if (damage.includes(false)) break;
        if damage.iter().any(|d| matches!(d, SpreadMoveDamageValue::Failed)) {
            break;
        }

        // if (hit > 1 && pokemon.status === 'slp' && (!isSleepUsable || this.battle.gen === 4)) break;
        if hit > 1 {
            let is_asleep = battle.pokemon_at(attacker_pos.0, attacker_pos.1)
                .map(|p| p.status.as_ref().map(|s| s.as_str() == "slp").unwrap_or(false))
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
                    .unwrap_or(SpreadMoveDamageValue::Damage(0));
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
        // TODO: Implement retargeting (needs addMove and retargetLastMove methods)

        // Handle multiaccuracy (Triple Kick)
        // if (target && move.multiaccuracy && hit > 1) {
        //     // ... accuracy calculation ...
        //     if (accuracy !== true && !this.battle.randomChance(accuracy, 100)) break;
        // }
        // TODO: Implement multiaccuracy logic

        // Modifies targetsCopy (which is why it's a copy)
        // [moveDamageThisHit, targetsCopy] = this.spreadMoveHit(targetsCopy, pokemon, move, moveData);
        let move_damage_this_hit = battle_actions::spread_move_hit(
            battle,
            &mut targets_copy,
            attacker_pos,
            active_move,
        );

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
        if !move_damage.iter().any(|d| !matches!(d, SpreadMoveDamageValue::Failed)) {
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
                SpreadMoveDamageValue::Damage(d) => *d,
                _ => 0,
            };
            if i < damage.len() {
                damage[i] = SpreadMoveDamageValue::Damage(dmg);
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
            let (hp_before_recoil, max_hp) = {
                let pokemon = match battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
                    Some(p) => p,
                    None => {
                        hit += 1;
                        continue;
                    }
                };
                (pokemon.hp, pokemon.max_hp)
            };

            let recoil_damage = (max_hp as f64 / 2.0).round() as i32;
            Pokemon::damage(battle, attacker_pos, recoil_damage, Some(attacker_pos), Some(&active_move.id));
            active_move.mindblown_recoil = false;

            let hp_after = battle.pokemon_at(attacker_pos.0, attacker_pos.1)
                .map(|p| p.hp)
                .unwrap_or(0);
            if hp_after <= max_hp / 2 && hp_before_recoil > max_hp / 2 {
                battle.run_event("EmergencyExit", Some(attacker_pos), Some(attacker_pos), None, None);
            }
        }

        // this.battle.eachEvent('Update');
        battle.each_event("Update");

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
        return vec![SpreadMoveDamageValue::Failed; damage.len()];
    }

    // if (nullDamage) damage.fill(false);
    if null_damage {
        damage = vec![SpreadMoveDamageValue::Failed; damage.len()];
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
                    crate::battle::Arg::Number(hit - 1),
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
        let (hp_before_recoil, max_hp, recoil_damage) = {
            let pokemon = match battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
                Some(p) => p,
                None => (0, 0, 0),
            };
            let recoil = crate::battle_actions::calc_recoil_damage::calc_recoil_damage(
                active_move.total_damage,
                active_move.id.as_str(),
                active_move.recoil,
                pokemon.max_hp,
            );
            (pokemon.hp, pokemon.max_hp, recoil)
        };

        if recoil_damage > 0 {
            Pokemon::damage(battle, attacker_pos, recoil_damage, Some(attacker_pos), Some(&ID::from("recoil")));

            let hp_after = battle.pokemon_at(attacker_pos.0, attacker_pos.1)
                .map(|p| p.hp)
                .unwrap_or(0);
            if hp_after <= max_hp / 2 && hp_before_recoil > max_hp / 2 {
                battle.run_event("EmergencyExit", Some(attacker_pos), Some(attacker_pos), None, None);
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
    if active_move.struggle_recoil {
        let (hp_before_recoil, max_hp, base_max_hp) = {
            let pokemon = match battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
                Some(p) => p,
                None => (0, 0, 0),
            };
            (pokemon.hp, pokemon.max_hp, pokemon.base_max_hp)
        };

        let recoil_damage = if battle.gen >= 5 {
            ((base_max_hp as f64 / 4.0).round() as i32).max(1)
        } else {
            ((max_hp / 4) as i32).max(1)
        };

        Pokemon::direct_damage(battle, attacker_pos, recoil_damage, Some(attacker_pos), Some(&ID::from("strugglerecoil")));

        let hp_after = battle.pokemon_at(attacker_pos.0, attacker_pos.1)
            .map(|p| p.hp)
            .unwrap_or(0);
        if hp_after <= max_hp / 2 && hp_before_recoil > max_hp / 2 {
            battle.run_event("EmergencyExit", Some(attacker_pos), Some(attacker_pos), None, None);
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
                Pokemon::got_attacked(battle, *target_pos, &active_move.id, damage_value, attacker_pos);

                if let Some(SpreadMoveDamageValue::Damage(_)) = damage_value {
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
    if !damage.iter().any(|d| matches!(d, SpreadMoveDamageValue::Damage(_))) {
        return damage;
    }

    // this.battle.eachEvent('Update');
    battle.each_event("Update");

    // this.afterMoveSecondaryEvent(targetsCopy.filter(val => !!val), pokemon, move);
    let valid_targets: Vec<(usize, usize)> = targets_copy.iter()
        .filter_map(|t| match t {
            SpreadMoveTarget::Target(pos) => Some(*pos),
            _ => None,
        })
        .collect();
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
                    SpreadMoveDamageValue::Damage(dmg) => *dmg,
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
                        (target.hp, target.max_hp, target.hurt_this_turn)
                    };

                    if target_hp > 0 {
                        let target_hp_before_damage = hurt_this_turn + cur_damage;
                        if target_hp <= target_max_hp / 2 && target_hp_before_damage > target_max_hp / 2 {
                            battle.run_event("EmergencyExit", Some(*target_pos), Some(attacker_pos), None, None);
                        }
                    }
                }
            }
        }
    }

    // return damage;
    damage
}
