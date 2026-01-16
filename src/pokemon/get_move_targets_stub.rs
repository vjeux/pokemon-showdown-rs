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
                None => return GetMoveTargetsResult { targets: vec![], pressure_targets: vec![] },
            };

        // Handle different target types
        match move_target.as_str() {
            "all" | "foeSide" | "allySide" | "allyTeam" => {
                // JS: if (!move.target.startsWith('foe')) targets.push(...this.alliesAndSelf());
                if !move_target.starts_with("foe") {
                    let allies = {
                        let pokemon = match battle.pokemon_at(user_pos.0, user_pos.1) {
                            Some(p) => p,
                            None => return GetMoveTargetsResult { targets: vec![], pressure_targets: vec![] },
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
                            None => return GetMoveTargetsResult { targets: vec![], pressure_targets: vec![] },
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
                        None => return GetMoveTargetsResult { targets: vec![], pressure_targets: vec![] },
                    };
                    pokemon.adjacent_allies(battle)
                };
                targets.extend(adjacent_allies);
                // falls through to allAdjacentFoes
                let adjacent_foes = {
                    let pokemon = match battle.pokemon_at(user_pos.0, user_pos.1) {
                        Some(p) => p,
                        None => return GetMoveTargetsResult { targets: vec![], pressure_targets: vec![] },
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
                        None => return GetMoveTargetsResult { targets: vec![], pressure_targets: vec![] },
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
                        None => return GetMoveTargetsResult { targets: vec![], pressure_targets: vec![] },
                    };
                    pokemon.allies_and_self(battle, false)
                };
            }
            _ => {
                // Default case - single target moves
                debug_elog!("[GET_MOVE_TARGETS] Default case: move_id={}, move_target={}, target={:?}", move_id, move_target, target);
                let selected_target = target;

                // JS: if (!target || (target.fainted && !target.isAlly(this)) && this.battle.gameType !== 'freeforall')
                if target.is_none()
                    || (battle.is_pokemon_fainted(target.unwrap())
                        && !battle.is_ally(target.unwrap(), user_pos)
                        && battle.game_type != GameType::FreeForAll)
                {
                    // JS: const possibleTarget = this.battle.getRandomTarget(this, move);
                    target = battle.get_random_target(user_pos.0, user_pos.1, &move_target);
                    if target.is_none() {
                        return GetMoveTargetsResult { targets: vec![], pressure_targets: vec![] };
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
                                    targets.push((ally_side, ally_pos));
                                    target = Some((ally_side, ally_pos));
                                } else {
                                    // Both targets alive, hit both
                                    targets.push((target_side, target_pos));
                                    target = Some((target_side, target_pos));
                                }
                            } else {
                                // target2 invalid, just use target
                                targets.push((target_side, target_pos));
                            }
                        } else {
                            // No target2, just use target
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
                if target.is_some_and(|t| battle.is_pokemon_fainted(t)) && !has_futuremove {
                    return GetMoveTargetsResult { targets: vec![], pressure_targets: vec![] };
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
                    None => return GetMoveTargetsResult { targets: vec![], pressure_targets: vec![] },
                };
                pokemon.foes(battle, true)
            };
        }

        GetMoveTargetsResult { targets, pressure_targets }
    }
}
