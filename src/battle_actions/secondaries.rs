//! BattleActions::secondaries - Apply secondary effects of a move
//!
//! 1:1 port of secondaries from battle-actions.ts

// JS Source:
// 	secondaries(targets: SpreadMoveTargets, source: Pokemon, move: ActiveMove, moveData: ActiveMove, isSelf?: boolean) {
// 		if (!moveData.secondaries) return;
// 		for (const target of targets) {
// 			if (target === false) continue;
// 			const secondaries: Dex.SecondaryEffect[] =
// 				this.battle.runEvent('ModifySecondaries', target, source, moveData, moveData.secondaries.slice());
// 			for (const secondary of secondaries) {
// 				const secondaryRoll = this.battle.random(100);
// 				// User stat boosts or target stat drops can possibly overflow if it goes beyond 256 in Gen 8 or prior
// 				const secondaryOverflow = (secondary.boosts || secondary.self) && this.battle.gen <= 8;
// 				if (typeof secondary.chance === 'undefined' ||
// 					secondaryRoll < (secondaryOverflow ? secondary.chance % 256 : secondary.chance)) {
// 					this.moveHit(target, source, move, secondary, true, isSelf);
// 				}
// 			}
// 		}
// 	}

use crate::*;
use crate::event::EventResult;
use crate::battle_actions::{SpreadMoveTargets, SpreadMoveTarget};

/// Apply secondary effects of a move
/// Equivalent to secondaries() in battle-actions.ts
///
/// JavaScript signature:
/// secondaries(targets: SpreadMoveTargets, source: Pokemon, move: ActiveMove, moveData: ActiveMove, isSelf?: boolean)
pub fn secondaries(
    battle: &mut Battle,
    targets: &SpreadMoveTargets,
    source_pos: (usize, usize),
    move_id: &ID,
    _is_self: bool,
) {
    eprintln!("[SECONDARIES] Called for move_id={}, targets.len={}", move_id.as_str(), targets.len());

    // if (!moveData.secondaries) return;
    let has_secondaries = {
        if let Some(ref active_move) = battle.active_move {
            !active_move.secondaries.is_empty()
        } else {
            false
        }
    };

    eprintln!("[SECONDARIES] has_secondaries={}", has_secondaries);

    if !has_secondaries {
        return;
    }

    // for (const target of targets) {
    for target in targets {
        // if (target === false) continue;
        let target_pos = match target {
            SpreadMoveTarget::Target(pos) => *pos,
            _ => continue,
        };

        // const secondaries: Dex.SecondaryEffect[] =
        //     this.battle.runEvent('ModifySecondaries', target, source, moveData, moveData.secondaries.slice());
        // Call ModifySecondaries event to allow abilities like Shield Dust to filter secondaries
        // The event can modify or filter the secondaries array IN-PLACE
        let secondaries = if battle.active_move.is_some() {
            // Fire ModifySecondaries event
            // In JavaScript, this event handler can modify the secondaries array and return it
            // In Rust, Shield Dust modifies battle.active_move.secondaries in-place using retain()
            let modify_result = battle.run_event("ModifySecondaries", Some(target_pos), Some(source_pos), Some(move_id), EventResult::Continue, false, false);

            // IMPORTANT: Clone AFTER the event, so we get the MODIFIED secondaries
            // Shield Dust's retain() has already filtered the array
            let secs = if let Some(ref active_move) = battle.active_move {
                active_move.secondaries.clone()
            } else {
                Vec::new()
            };

            // If ModifySecondaries event returns a falsy value (like EventResult::Null or EventResult::Boolean(false)),
            // it means the secondaries should be blocked entirely
            match modify_result {
                EventResult::Number(0) | EventResult::Boolean(false) | EventResult::Null => {
                    // Event returned falsy value, skip all secondaries for this target
                    Vec::new()
                }
                _ => {
                    // Event returned truthy value or didn't interfere, use the modified secondaries
                    secs
                }
            }
        } else {
            continue;
        };

        // for (const secondary of secondaries) {
        for secondary in secondaries {
            eprintln!("[SECONDARIES] Processing secondary: chance={:?}, volatile_status={:?}, target={:?}",
                secondary.chance, secondary.volatile_status, target_pos);

            // const secondaryRoll = this.battle.random(100);
            let secondary_roll = battle.random(100);

            // const secondaryOverflow = (secondary.boosts || secondary.self) && this.battle.gen <= 8;
            let has_boosts = secondary.boosts.is_some();
            let has_self = secondary.self_effect;
            let secondary_overflow = (has_boosts || has_self) && battle.gen <= 8;

            // if (typeof secondary.chance === 'undefined' ||
            //     secondaryRoll < (secondaryOverflow ? secondary.chance % 256 : secondary.chance)) {
            let should_apply = match secondary.chance {
                None => true, // chance is undefined, always apply
                Some(chance) => {
                    let effective_chance = if secondary_overflow {
                        chance % 256
                    } else {
                        chance
                    };
                    secondary_roll < effective_chance
                }
            };

            if should_apply {
                // this.moveHit(target, source, move, secondary, true, isSelf);
                // Implement all secondary effects from moveHit

                // Apply stat boosts
                if let Some(ref boosts) = secondary.boosts {
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

                    battle.boost(&boost_array, target_pos, Some(source_pos), None, true, false);
                }

                // Apply status from secondary effect
                // JS: if (moveData.status) {
                //     hitResult = target.setStatus(moveData.status, source, move);
                // }
                if let Some(status_name) = &secondary.status {
                    let status_id = crate::dex_data::ID::new(status_name);
                    let _applied = Pokemon::set_status(battle, target_pos, status_id, Some(source_pos), Some(move_id), false);
                }

                // Apply volatile status from secondary effect
                // JS: if (moveData.volatileStatus) {
                //     hitResult = target.addVolatile(moveData.volatileStatus, source, move);
                // }
                if let Some(volatile_status_name) = &secondary.volatile_status {
                    eprintln!("[SECONDARIES] Applying volatile_status='{}' to target={:?}", volatile_status_name, target_pos);
                    let volatile_id = crate::dex_data::ID::new(volatile_status_name);
                    Pokemon::add_volatile(battle, target_pos, volatile_id, Some(source_pos), Some(move_id), None, None);
                }

                // Apply side condition from secondary effect
                // JS: if (moveData.sideCondition) {
                //     hitResult = target.side.addSideCondition(moveData.sideCondition, source, move);
                // }
                if let Some(side_condition_name) = &secondary.side_condition {
                    let side_condition_id = crate::dex_data::ID::new(side_condition_name);
                    let _applied = battle.sides[target_pos.0].add_side_condition(side_condition_id, None);
                }

                // Apply slot condition from secondary effect
                // JS: if (moveData.slotCondition) {
                //     hitResult = target.side.addSlotCondition(target, moveData.slotCondition, source, move);
                // }
                if let Some(slot_condition_name) = &secondary.slot_condition {
                    let slot_condition_id = crate::dex_data::ID::new(slot_condition_name);
                    let _applied = battle.sides[target_pos.0].add_slot_condition(target_pos.1, slot_condition_id, None);
                }

                // Apply pseudo weather from secondary effect
                // JS: if (moveData.pseudoWeather) {
                //     hitResult = this.battle.field.addPseudoWeather(moveData.pseudoWeather, source, move);
                // }
                if let Some(pseudo_weather_name) = &secondary.pseudo_weather {
                    let pseudo_weather_id = crate::dex_data::ID::new(pseudo_weather_name);
                    let _applied = battle.add_pseudo_weather(pseudo_weather_id, None);
                }

                // Apply terrain from secondary effect
                // JS: if (moveData.terrain) {
                //     hitResult = this.battle.field.setTerrain(moveData.terrain, source, move);
                // }
                if let Some(terrain_name) = &secondary.terrain {
                    let terrain_id = crate::dex_data::ID::new(terrain_name);
                    let _applied = battle.set_terrain(terrain_id, None);
                }

                // Apply weather from secondary effect
                // JS: if (moveData.weather) {
                //     hitResult = this.battle.field.setWeather(moveData.weather, source, move);
                // }
                if let Some(weather_name) = &secondary.weather {
                    let weather_id = crate::dex_data::ID::new(weather_name);
                    let _applied = battle.set_weather(weather_id, None, None);
                }
            }
        }
    }
}
