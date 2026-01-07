//! BattleActions::selfDrops - Apply self stat drops after a move
//!
//! 1:1 port of selfDrops from battle-actions.ts

// JS Source:
// 	selfDrops(
// 		targets: SpreadMoveTargets, source: Pokemon,
// 		move: ActiveMove, moveData: ActiveMove, isSecondary?: boolean
// 	) {
// 		for (const target of targets) {
// 			if (target === false) continue;
// 			if (moveData.self && !move.selfDropped) {
// 				if (!isSecondary && moveData.self.boosts) {
// 					const secondaryRoll = this.battle.random(100);
// 					if (typeof moveData.self.chance === 'undefined' || secondaryRoll < moveData.self.chance) {
// 						this.moveHit(source, source, move, moveData.self, isSecondary, true);
// 					}
// 					if (!move.multihit) move.selfDropped = true;
// 				} else {
// 					this.moveHit(source, source, move, moveData.self, isSecondary, true);
// 				}
// 			}
// 		}
// 	}

use crate::*;
use crate::battle_actions::{SpreadMoveTargets, SpreadMoveTarget};
use crate::battle::Effect;

/// Apply self stat drops after a move
/// Equivalent to selfDrops() in battle-actions.ts
///
/// JavaScript signature:
/// selfDrops(targets: SpreadMoveTargets, source: Pokemon, move: ActiveMove, moveData: ActiveMove, isSecondary?: boolean)
pub fn self_drops(
    battle: &mut Battle,
    targets: &SpreadMoveTargets,
    source_pos: (usize, usize),
    _move_id: &ID,
    is_secondary: bool,
) {
    eprintln!("[SELF_DROPS] Called with source_pos={:?}, is_secondary={}", source_pos, is_secondary);

    // Get moveData.self from active_move
    let (has_self_data, has_boosts, self_chance, is_multihit, self_dropped) = {
        if let Some(ref active_move) = battle.active_move {
            let has_self = active_move.self_effect.is_some();
            let has_boosts = active_move.self_effect.as_ref()
                .and_then(|s| s.boosts.as_ref())
                .is_some();
            let chance = active_move.self_effect.as_ref()
                .and_then(|s| s.chance);
            let multihit = active_move.multi_hit.is_some();
            let dropped = active_move.self_dropped;
            eprintln!("[SELF_DROPS] has_self={}, has_boosts={}, self_chance={:?}, is_multihit={}, self_dropped={}",
                has_self, has_boosts, chance, multihit, dropped);
            (has_self, has_boosts, chance, multihit, dropped)
        } else {
            eprintln!("[SELF_DROPS] No active_move!");
            return;
        }
    };

    // for (const target of targets) {
    for target in targets {
        // if (target === false) continue;
        if matches!(target, SpreadMoveTarget::Failed) {
            continue;
        }

        // if (moveData.self && !move.selfDropped) {
        if has_self_data && !self_dropped {
            eprintln!("[SELF_DROPS] Processing self effect");
            // if (!isSecondary && moveData.self.boosts) {
            if !is_secondary && has_boosts {
                eprintln!("[SELF_DROPS] Making random(100) call for self effect with boosts");
                // const secondaryRoll = this.battle.random(100);
                let secondary_roll = battle.random(100);
                eprintln!("[SELF_DROPS] secondary_roll={}", secondary_roll);

                // if (typeof moveData.self.chance === 'undefined' || secondaryRoll < moveData.self.chance) {
                let should_apply = match self_chance {
                    None => true, // chance is undefined, always apply
                    Some(chance) => secondary_roll < chance,
                };

                if should_apply {
                    // this.moveHit(source, source, move, moveData.self, isSecondary, true);
                    // Apply all self effects to the source Pokemon

                    if let Some(ref active_move) = battle.active_move.clone() {
                        if let Some(ref self_data) = active_move.self_effect {
                            // Apply stat boosts
                            if let Some(ref boosts) = self_data.boosts {
                                let mut boost_array = Vec::new();
                                if boosts.atk != 0 {
                                    boost_array.push(("atk", boosts.atk));
                                }
                                if boosts.def != 0 {
                                    boost_array.push(("def", boosts.def));
                                }
                                if boosts.spa != 0 {
                                    boost_array.push(("spa", boosts.spa));
                                }
                                if boosts.spd != 0 {
                                    boost_array.push(("spd", boosts.spd));
                                }
                                if boosts.spe != 0 {
                                    boost_array.push(("spe", boosts.spe));
                                }
                                if boosts.accuracy != 0 {
                                    boost_array.push(("accuracy", boosts.accuracy));
                                }
                                if boosts.evasion != 0 {
                                    boost_array.push(("evasion", boosts.evasion));
                                }

                                battle.boost(&boost_array, source_pos, Some(source_pos), None, false, true);
                            }

                            // Apply status from self effect (to source)
                            // JS: if (moveData.status) {
                            //     hitResult = target.setStatus(moveData.status, source, move);
                            // }
                            if let Some(ref status_name) = self_data.status {
                                let status_id = crate::dex_data::ID::new(status_name);
                                let _applied = Pokemon::set_status(battle, source_pos, status_id, None, None, false);
                            }

                            // Apply volatile status from self effect (to source)
                            // JS: if (moveData.volatileStatus) {
                            //     hitResult = target.addVolatile(moveData.volatileStatus, source, move);
                            // }
                            if let Some(ref volatile_status_name) = self_data.volatile_status {
                                let volatile_id = crate::dex_data::ID::new(volatile_status_name);
                                Pokemon::add_volatile(battle, source_pos, volatile_id, None, None, None, None);
                            }

                            // Apply side condition from self effect (to source's side)
                            // JS: if (moveData.sideCondition) {
                            //     hitResult = target.side.addSideCondition(moveData.sideCondition, source, move);
                            // }
                            if let Some(ref side_condition_name) = self_data.side_condition {
                                let side_condition_id = crate::dex_data::ID::new(side_condition_name);
                                // Use Battle::add_side_condition with source tracking (not Side::add_side_condition)
                                // This ensures durationCallback and SideStart callbacks are called
                                let move_effect = Effect::move_(_move_id.clone());
                                let _applied = battle.add_side_condition(
                                    source_pos.0,           // side_idx
                                    side_condition_id,      // condition_id
                                    Some(source_pos),       // source
                                    Some(&move_effect),     // sourceEffect (move)
                                );
                            }

                            // Apply slot condition from self effect (to source's slot)
                            // JS: if (moveData.slotCondition) {
                            //     hitResult = target.side.addSlotCondition(target, moveData.slotCondition, source, move);
                            // }
                            if let Some(ref slot_condition_name) = self_data.slot_condition {
                                let slot_condition_id = crate::dex_data::ID::new(slot_condition_name);
                                let _applied = battle.sides[source_pos.0].add_slot_condition(source_pos.1, slot_condition_id, None);
                            }

                            // Apply pseudo weather from self effect
                            // JS: if (moveData.pseudoWeather) {
                            //     hitResult = this.battle.field.addPseudoWeather(moveData.pseudoWeather, source, move);
                            // }
                            if let Some(ref pseudo_weather_name) = self_data.pseudo_weather {
                                let pseudo_weather_id = crate::dex_data::ID::new(pseudo_weather_name);
                                let _applied = battle.add_pseudo_weather(pseudo_weather_id, None);
                            }

                            // Apply terrain from self effect
                            // JS: if (moveData.terrain) {
                            //     hitResult = this.battle.field.setTerrain(moveData.terrain, source, move);
                            // }
                            if let Some(ref terrain_name) = self_data.terrain {
                                let terrain_id = crate::dex_data::ID::new(terrain_name);
                                let terrain_effect = Some(Effect::move_(_move_id.clone()));
                                let _applied = battle.set_terrain(terrain_id, None, terrain_effect);
                            }

                            // Apply weather from self effect
                            // JS: if (moveData.weather) {
                            //     hitResult = this.battle.field.setWeather(moveData.weather, source, move);
                            // }
                            if let Some(ref weather_name) = self_data.weather {
                                let weather_id = crate::dex_data::ID::new(weather_name);
                                let _applied = battle.set_weather(weather_id, None, None);
                            }
                        }
                    }
                }

                // if (!move.multihit) move.selfDropped = true;
                if !is_multihit {
                    if let Some(ref mut active_move) = battle.active_move {
                        active_move.self_dropped = true;
                    }
                }
            } else {
                // this.moveHit(source, source, move, moveData.self, isSecondary, true);
                // Apply all self effects to the source Pokemon

                if let Some(ref active_move) = battle.active_move.clone() {
                    if let Some(ref self_data) = active_move.self_effect {
                        // Apply stat boosts
                        if let Some(ref boosts) = self_data.boosts {
                            let mut boost_array = Vec::new();
                            if boosts.atk != 0 {
                                boost_array.push(("atk", boosts.atk));
                            }
                            if boosts.def != 0 {
                                boost_array.push(("def", boosts.def));
                            }
                            if boosts.spa != 0 {
                                boost_array.push(("spa", boosts.spa));
                            }
                            if boosts.spd != 0 {
                                boost_array.push(("spd", boosts.spd));
                            }
                            if boosts.spe != 0 {
                                boost_array.push(("spe", boosts.spe));
                            }
                            if boosts.accuracy != 0 {
                                boost_array.push(("accuracy", boosts.accuracy));
                            }
                            if boosts.evasion != 0 {
                                boost_array.push(("evasion", boosts.evasion));
                            }

                            battle.boost(&boost_array, source_pos, Some(source_pos), None, false, true);
                        }

                        // Apply status from self effect (to source)
                        // JS: if (moveData.status) {
                        //     hitResult = target.setStatus(moveData.status, source, move);
                        // }
                        if let Some(status_name) = &self_data.status {
                            let status_id = crate::dex_data::ID::new(status_name);
                            let _applied = Pokemon::set_status(battle, source_pos, status_id, None, None, false);
                        }

                        // Apply volatile status from self effect (to source)
                        // JS: if (moveData.volatileStatus) {
                        //     hitResult = target.addVolatile(moveData.volatileStatus, source, move);
                        // }
                        if let Some(volatile_status_name) = &self_data.volatile_status {
                            let volatile_id = crate::dex_data::ID::new(volatile_status_name);
                            Pokemon::add_volatile(battle, source_pos, volatile_id, None, None, None, None);
                        }

                        // Apply side condition from self effect (to source's side)
                        // JS: if (moveData.sideCondition) {
                        //     hitResult = target.side.addSideCondition(moveData.sideCondition, source, move);
                        // }
                        if let Some(ref side_condition_name) = self_data.side_condition {
                            let side_condition_id = crate::dex_data::ID::new(side_condition_name);
                            // Use Battle::add_side_condition with source tracking (not Side::add_side_condition)
                            // This ensures durationCallback and SideStart callbacks are called
                            let move_effect = Effect::move_(_move_id.clone());
                            let _applied = battle.add_side_condition(
                                source_pos.0,           // side_idx
                                side_condition_id,      // condition_id
                                Some(source_pos),       // source
                                Some(&move_effect),     // sourceEffect (move)
                            );
                        }

                        // Apply slot condition from self effect (to source's slot)
                        // JS: if (moveData.slotCondition) {
                        //     hitResult = target.side.addSlotCondition(target, moveData.slotCondition, source, move);
                        // }
                        if let Some(ref slot_condition_name) = self_data.slot_condition {
                            let slot_condition_id = crate::dex_data::ID::new(slot_condition_name);
                            let _applied = battle.sides[source_pos.0].add_slot_condition(source_pos.1, slot_condition_id, None);
                        }

                        // Apply pseudo weather from self effect
                        // JS: if (moveData.pseudoWeather) {
                        //     hitResult = this.battle.field.addPseudoWeather(moveData.pseudoWeather, source, move);
                        // }
                        if let Some(ref pseudo_weather_name) = self_data.pseudo_weather {
                            let pseudo_weather_id = crate::dex_data::ID::new(pseudo_weather_name);
                            let _applied = battle.add_pseudo_weather(pseudo_weather_id, None);
                        }

                        // Apply terrain from self effect
                        // JS: if (moveData.terrain) {
                        //     hitResult = this.battle.field.setTerrain(moveData.terrain, source, move);
                        // }
                        if let Some(ref terrain_name) = self_data.terrain {
                            let terrain_id = crate::dex_data::ID::new(terrain_name);
                            let terrain_effect = Some(Effect::move_(_move_id.clone()));
                            let _applied = battle.set_terrain(terrain_id, None, terrain_effect);
                        }

                        // Apply weather from self effect
                        // JS: if (moveData.weather) {
                        //     hitResult = this.battle.field.setWeather(moveData.weather, source, move);
                        // }
                        if let Some(ref weather_name) = self_data.weather {
                            let weather_id = crate::dex_data::ID::new(weather_name);
                            let _applied = battle.set_weather(weather_id, None, None);
                        }
                    }
                }

                //  INFRASTRUCTURE FIX: Call run_move_effects with self effect data
                // JavaScript calls: this.moveHit(source, source, move, moveData.self, isSecondary, true);
                // moveHit internally calls runMoveEffects which triggers onHit callbacks
                // We need to do the same in Rust to trigger self.onHit callbacks like gmaxvolcalith
                eprintln!("[SELF_DROPS] Checking if should call run_move_effects for self callbacks");
                if let Some(ref active_move) = battle.active_move.clone() {
                    eprintln!("[SELF_DROPS] active_move.id={}", active_move.id.as_str());
                    if let Some(ref self_data) = active_move.self_effect {
                        eprintln!("[SELF_DROPS] Calling run_move_effects for self.onHit callback, move_id={}", active_move.id.as_str());
                        // Create a temporary ActiveMove from self_effect data for runMoveEffects
                        // This allows self effect callbacks (like onHit) to be triggered
                        let mut self_active_move = active_move.clone();
                        // Override fields with self effect data where applicable
                        self_active_move.boosts = self_data.boosts.clone();
                        self_active_move.status = self_data.status.clone();
                        self_active_move.volatile_status = self_data.volatile_status.clone();
                        self_active_move.side_condition = self_data.side_condition.clone();
                        self_active_move.slot_condition = self_data.slot_condition.clone();
                        self_active_move.pseudo_weather = self_data.pseudo_weather.clone();
                        self_active_move.terrain = self_data.terrain.clone();
                        self_active_move.weather = self_data.weather.clone();

                        // Create targets array with source as the target (self-targeting)
                        let self_targets = vec![crate::battle_actions::SpreadMoveTarget::Target(source_pos)];
                        let self_damages = vec![crate::battle_actions::DamageResult::Undefined];

                        // Call run_move_effects with isSelf=true
                        let _result = crate::battle_actions::run_move_effects(
                            battle,
                            self_damages,
                            &self_targets,
                            source_pos,
                            &active_move,  // Original move as active_move
                            &self_active_move,  // Self effect data as move_data
                            is_secondary,
                            true,  // isSelf=true
                        );
                    }
                }
            }
        }
    }
}
