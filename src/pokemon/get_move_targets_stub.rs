// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::event::EventResult;
use crate::pokemon::GetMoveTargetsResult;
use crate::battle::Effect;
use crate::battle_actions::ActiveMove;

impl Pokemon {

    /// Get move targets and pressure targets
    /// Equivalent to pokemon.ts getMoveTargets()
    ///
    // 	getMoveTargets(move: ActiveMove, target: Pokemon): { targets: Pokemon[], pressureTargets: Pokemon[] } {
    // 		let targets: Pokemon[] = [];
    //
    // 		switch (move.target) {
    // 		case 'all':
    // 		case 'foeSide':
    // 		case 'allySide':
    // 		case 'allyTeam':
    // 			if (!move.target.startsWith('foe')) {
    // 				targets.push(...this.alliesAndSelf());
    // 			}
    // 			if (!move.target.startsWith('ally')) {
    // 				targets.push(...this.foes(true));
    // 			}
    // 			if (targets.length && !targets.includes(target)) {
    // 				this.battle.retargetLastMove(targets[targets.length - 1]);
    // 			}
    // 			break;
    // 		case 'allAdjacent':
    // 			targets.push(...this.adjacentAllies());
    // 			// falls through
    // 		case 'allAdjacentFoes':
    // 			targets.push(...this.adjacentFoes());
    // 			if (targets.length && !targets.includes(target)) {
    // 				this.battle.retargetLastMove(targets[targets.length - 1]);
    // 			}
    // 			break;
    // 		case 'allies':
    // 			targets = this.alliesAndSelf();
    // 			break;
    // 		default:
    // 			const selectedTarget = target;
    // 			if (!target || (target.fainted && !target.isAlly(this)) && this.battle.gameType !== 'freeforall') {
    // 				// If a targeted foe faints, the move is retargeted
    // 				const possibleTarget = this.battle.getRandomTarget(this, move);
    // 				if (!possibleTarget) return { targets: [], pressureTargets: [] };
    // 				target = possibleTarget;
    // 			}
    // 			if (this.battle.activePerHalf > 1 && !move.tracksTarget) {
    // 				const isCharging = move.flags['charge'] && !this.volatiles['twoturnmove'] &&
    // 					!(move.id.startsWith('solarb') && ['sunnyday', 'desolateland'].includes(this.effectiveWeather())) &&
    // 					!(move.id === 'electroshot' && ['raindance', 'primordialsea'].includes(this.effectiveWeather())) &&
    // 					!(this.hasItem('powerherb') && move.id !== 'skydrop');
    // 				if (!isCharging && !(move.id === 'pursuit' && (target.beingCalledBack || target.switchFlag))) {
    // 					target = this.battle.priorityEvent('RedirectTarget', this, this, move, target);
    // 				}
    // 			}
    // 			if (move.smartTarget) {
    // 				targets = this.getSmartTargets(target, move);
    // 				target = targets[0];
    // 			} else {
    // 				targets.push(target);
    // 			}
    // 			if (target.fainted && !move.flags['futuremove']) {
    // 				return { targets: [], pressureTargets: [] };
    // 			}
    // 			if (selectedTarget !== target) {
    // 				this.battle.retargetLastMove(target);
    // 			}
    // 		}
    //
    // 		// Resolve apparent targets for Pressure.
    // 		let pressureTargets = targets;
    // 		if (move.target === 'foeSide') {
    // 			pressureTargets = [];
    // 		}
    // 		if (move.flags['mustpressure']) {
    // 			pressureTargets = this.foes();
    // 		}
    //
    // 		return { targets, pressureTargets };
    // 	}
    //
    // Note: In Rust, Pokemon doesn't have a reference to Battle (borrow checker),
    // so we take Battle as a parameter instead of accessing this.battle
    // This is an associated function (not instance method) because we need &mut Battle
    // but can't borrow pokemon from battle first due to borrow checker
    pub fn get_move_targets(
        battle: &mut Battle,
        user_pos: (usize, usize),
        active_move: &ActiveMove,
        mut target: Option<(usize, usize)>,
    ) -> GetMoveTargetsResult {
        let move_id = &active_move.id;
        let mut targets: Vec<(usize, usize)> = Vec::new();

        // Get move data to access target and flags
        let (move_target, has_mustpressure, has_futuremove, has_smart_target) =
            match battle.dex.moves().get(move_id.as_str()) {
                Some(m) => (
                    m.target.clone(),
                    m.flags.contains_key("mustpressure"),
                    m.flags.contains_key("futuremove"),
                    m.smart_target.unwrap_or(false),
                ),
                None => return GetMoveTargetsResult { targets: vec![], pressure_targets: vec![], should_clear_smart_target: false },
            };

        // Track whether smartTarget should be set to false (when only one target is found)
        // This is set in the default arm when handling smartTarget moves in singles
        let mut should_clear_smart_target = false;

        // Handle different target types
        match move_target.as_str() {
            "all" | "foeSide" | "allySide" | "allyTeam" => {
                // JS: if (!move.target.startsWith('foe')) targets.push(...this.alliesAndSelf());
                if !move_target.starts_with("foe") {
                    let allies = {
                        let pokemon = match battle.pokemon_at(user_pos.0, user_pos.1) {
                            Some(p) => p,
                            None => return GetMoveTargetsResult { targets: vec![], pressure_targets: vec![], should_clear_smart_target: false },
                        };
                        pokemon.allies_and_self(battle, true)
                    };
                    targets.extend(allies);
                }
                // JS: if (!move.target.startsWith('ally')) targets.push(...this.foes(true));
                if !move_target.starts_with("ally") {
                    let foes = {
                        let pokemon = match battle.pokemon_at(user_pos.0, user_pos.1) {
                            Some(p) => p,
                            None => return GetMoveTargetsResult { targets: vec![], pressure_targets: vec![], should_clear_smart_target: false },
                        };
                        pokemon.foes(battle, true)
                    };
                    targets.extend(foes);
                }
                // JS: if (targets.length && !targets.includes(target)) this.battle.retargetLastMove(targets[targets.length - 1]);
                if !targets.is_empty() && !target.is_some_and(|t| targets.contains(&t)) {
                    if let Some(&last_target) = targets.last() {
                        battle.retarget_last_move(last_target);
                    }
                }
            }
            "allAdjacent" => {
                // JS: targets.push(...this.adjacentAllies());
                let adjacent_allies = {
                    let pokemon = match battle.pokemon_at(user_pos.0, user_pos.1) {
                        Some(p) => p,
                        None => return GetMoveTargetsResult { targets: vec![], pressure_targets: vec![], should_clear_smart_target: false },
                    };
                    pokemon.adjacent_allies(battle)
                };
                targets.extend(adjacent_allies);
                // falls through to allAdjacentFoes
                let adjacent_foes = {
                    let pokemon = match battle.pokemon_at(user_pos.0, user_pos.1) {
                        Some(p) => p,
                        None => return GetMoveTargetsResult { targets: vec![], pressure_targets: vec![], should_clear_smart_target: false },
                    };
                    pokemon.adjacent_foes(battle)
                };
                targets.extend(adjacent_foes);
                if !targets.is_empty() && !target.is_some_and(|t| targets.contains(&t)) {
                    if let Some(&last_target) = targets.last() {
                        battle.retarget_last_move(last_target);
                    }
                }
            }
            "allAdjacentFoes" => {
                // JS: targets.push(...this.adjacentFoes());
                let adjacent_foes = {
                    let pokemon = match battle.pokemon_at(user_pos.0, user_pos.1) {
                        Some(p) => p,
                        None => return GetMoveTargetsResult { targets: vec![], pressure_targets: vec![], should_clear_smart_target: false },
                    };
                    pokemon.adjacent_foes(battle)
                };
                targets.extend(adjacent_foes);
                if !targets.is_empty() && !target.is_some_and(|t| targets.contains(&t)) {
                    if let Some(&last_target) = targets.last() {
                        battle.retarget_last_move(last_target);
                    }
                }
            }
            "allies" => {
                // JS: targets = this.alliesAndSelf();
                targets = {
                    let pokemon = match battle.pokemon_at(user_pos.0, user_pos.1) {
                        Some(p) => p,
                        None => return GetMoveTargetsResult { targets: vec![], pressure_targets: vec![], should_clear_smart_target: false },
                    };
                    pokemon.allies_and_self(battle, false)
                };
            }
            _ => {
                // Default case - single target moves
                debug_elog!("[GET_MOVE_TARGETS] Default case: move_id={}, move_target={}, target={:?}", move_id, move_target, target);
                let selected_target = target;

                // Check if target is fainted and needs retargeting
                let target_fainted = target.is_some() && battle.is_pokemon_fainted(target.unwrap());
                let target_is_ally = target.is_some() && battle.is_ally(target.unwrap(), user_pos);
                debug_elog!("[GET_MOVE_TARGETS] target_fainted={}, target_is_ally={}, game_type={:?}", target_fainted, target_is_ally, battle.game_type);

                // JS: if (!target || (target.fainted && !target.isAlly(this)) && this.battle.gameType !== 'freeforall')
                if target.is_none()
                    || (battle.is_pokemon_fainted(target.unwrap())
                        && !battle.is_ally(target.unwrap(), user_pos)
                        && battle.game_type != GameType::FreeForAll)
                {
                    // JS: const possibleTarget = this.battle.getRandomTarget(this, move);
                    debug_elog!("[GET_MOVE_TARGETS] Need to retarget, calling get_random_target");
                    target = battle.get_random_target(user_pos.0, user_pos.1, &move_target);
                    debug_elog!("[GET_MOVE_TARGETS] get_random_target returned {:?}", target);
                    if target.is_none() {
                        debug_elog!("[GET_MOVE_TARGETS] No valid target found, returning empty");
                        return GetMoveTargetsResult { targets: vec![], pressure_targets: vec![], should_clear_smart_target: false };
                    }
                }

                // JS: if (this.battle.activePerHalf > 1 && !move.tracksTarget) {
                //       target = this.battle.priorityEvent('RedirectTarget', this, this, move, target);
                //     }
                if battle.active_per_half > 1 {
                    // Get move data to check tracksTarget
                    if let Some(move_data) = battle.dex.moves().get(move_id.as_str()) {
                        if !move_data.tracks_target.unwrap_or(false) {
                            // Encode current target position for relay variable
                            if let Some((target_side, target_pos)) = target {
                                let encoded_target = (target_side as i32 * 10) + target_pos as i32;

                                // Call RedirectTarget priority event
                                let redirect_result = battle.priority_event(
                                    "RedirectTarget",
                                    Some(user_pos),
                                    Some(user_pos),
                                    Some(&Effect::move_(move_id.clone())),
                                    EventResult::Number(encoded_target),
                                );

                                if let EventResult::Number(new_encoded) = redirect_result {
                                    // Decode the new target position
                                    let new_side = (new_encoded / 10) as usize;
                                    let new_pos = (new_encoded % 10) as usize;
                                    target = Some((new_side, new_pos));
                                }
                            }
                        }
                    }
                }

                // JS: if (move.smartTarget) {
                //       targets = this.getSmartTargets(target, move);
                //       target = targets[0];
                //     }
                // Smart targeting for Dragon Darts: if target fainted, retarget to adjacent ally
                if has_smart_target {
                    if let Some((target_side, target_pos)) = target {
                        // Get target's first adjacent ally
                        let target2 = {
                            if let Some(target_pokemon) = battle.pokemon_at(target_side, target_pos) {
                                let adjacent_allies = target_pokemon.adjacent_allies(battle);
                                adjacent_allies.first().copied()
                            } else {
                                None
                            }
                        };

                        // Check if target2 is valid (exists, is not self, has HP)
                        if let Some((ally_side, ally_pos)) = target2 {
                            let target2_is_self = ally_side == user_pos.0 && ally_pos == user_pos.1;
                            let target2_has_hp = !battle.is_pokemon_fainted((ally_side, ally_pos));
                            let target_has_hp = !battle.is_pokemon_fainted((target_side, target_pos));

                            if !target2_is_self && target2_has_hp {
                                if !target_has_hp {
                                    // Target fainted, use target2 instead
                                    // JS: move.smartTarget = false (only one target)
                                    should_clear_smart_target = true;
                                    targets.push((ally_side, ally_pos));
                                    target = Some((ally_side, ally_pos));
                                } else {
                                    // Both targets alive, hit both (Dragon Darts in doubles)
                                    // JS: return [target, target2]
                                    targets.push((target_side, target_pos));
                                    targets.push((ally_side, ally_pos));
                                    target = Some((target_side, target_pos));
                                }
                            } else {
                                // target2 invalid (is self or fainted), just use target
                                // JS: move.smartTarget = false
                                should_clear_smart_target = true;
                                targets.push((target_side, target_pos));
                            }
                        } else {
                            // No target2 (singles or no adjacent ally), just use target
                            // JS: move.smartTarget = false
                            should_clear_smart_target = true;
                            targets.push((target_side, target_pos));
                        }
                    }
                } else {
                    // Not smart targeting, just add target
                    if let Some(t) = target {
                        targets.push(t);
                    }
                }

                // JS: if (target.fainted && !move.flags['futuremove']) return { targets: [], pressureTargets: [] };
                let target_still_fainted = target.is_some_and(|t| battle.is_pokemon_fainted(t));
                debug_elog!("[GET_MOVE_TARGETS] Final check: target={:?}, target_still_fainted={}, has_futuremove={}", target, target_still_fainted, has_futuremove);
                if target_still_fainted && !has_futuremove {
                    debug_elog!("[GET_MOVE_TARGETS] Target is still fainted, returning empty");
                    return GetMoveTargetsResult { targets: vec![], pressure_targets: vec![], should_clear_smart_target };
                }

                // JS: if (selectedTarget !== target) this.battle.retargetLastMove(target);
                if selected_target != target {
                    if let Some(t) = target {
                        battle.retarget_last_move(t);
                    }
                }
            }
        }

        debug_elog!("[GET_MOVE_TARGETS] Returning targets={:?}", targets);

        // Resolve apparent targets for Pressure
        // JS: let pressureTargets = targets;
        let mut pressure_targets = targets.clone();

        // JS: if (move.target === 'foeSide') pressureTargets = [];
        if move_target == "foeSide" {
            pressure_targets.clear();
        }

        // JS: if (move.flags['mustpressure']) pressureTargets = this.foes();
        if has_mustpressure {
            pressure_targets = {
                let pokemon = match battle.pokemon_at(user_pos.0, user_pos.1) {
                    Some(p) => p,
                    None => return GetMoveTargetsResult { targets: vec![], pressure_targets: vec![], should_clear_smart_target: false },
                };
                pokemon.foes(battle, true)
            };
        }

        GetMoveTargetsResult { targets, pressure_targets, should_clear_smart_target }
    }
}
