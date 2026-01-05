//! BattleActions::runMoveEffects - Run move effects (boosts, healing, status, etc.)
//!
//! 1:1 port of runMoveEffects from battle-actions.ts:1201

use crate::*;
use crate::battle_actions::{SpreadMoveDamage, SpreadMoveTargets, SpreadMoveTarget};
use crate::event::EventResult;

/// Run move effects (boosts, healing, status, etc.)
/// Equivalent to runMoveEffects() in battle-actions.ts:1201
pub fn run_move_effects(
    battle: &mut Battle,
    damages: SpreadMoveDamage,
    targets: &SpreadMoveTargets,
    _source_pos: (usize, usize),
    move_data: &crate::dex::MoveData,
    _is_secondary: bool,
    _is_self: bool,
) -> SpreadMoveDamage {
    eprintln!("[RUN_MOVE_EFFECTS] Called for move: {:?}, has status: {:?}", move_data.id, move_data.status);
    let result_damages = damages;

    for target in targets.iter() {
        let target_pos = match target {
            SpreadMoveTarget::Target(pos) => *pos,
            _ => continue,
        };

        // Apply boosts
        if let Some(ref boosts_map) = move_data.boosts {
            // Convert HashMap<String, i32> to boost array for battle.boost()
            let mut boost_array = Vec::new();
            for (stat_name, &value) in boosts_map.iter() {
                if value != 0 {
                    boost_array.push((stat_name.as_str(), value as i8));
                }
            }

            // Use battle.boost() to apply boosts (fires events, handles boost limits, etc.)
            // JavaScript: this.boost(moveData.boosts, target, source, move);
            if !boost_array.is_empty() {
                battle.boost(&boost_array, target_pos, Some(_source_pos), None, _is_secondary, _is_self);
            }
        }

        // Apply healing
        // CRITICAL: Check active_move.heal first!
        // Moves like Present modify active_move.heal in their onModifyMove callback
        // JavaScript passes the same move object around, but in Rust we have separate move_data and active_move
        let heal_tuple = if let Some(ref active_move) = battle.active_move {
            active_move.heal.or(move_data.heal)
        } else {
            move_data.heal
        };

        if let Some((heal_num, heal_denom)) = heal_tuple {
            let target_maxhp = if let Some(side) = battle.sides.get(target_pos.0) {
                if let Some(pokemon) = side.pokemon.get(target_pos.1) {
                    pokemon.maxhp
                } else {
                    0
                }
            } else {
                0
            };

            if target_maxhp > 0 {
                let heal_amount = target_maxhp * heal_num / heal_denom;
                // Apply healing
                let (current_hp, max_hp) = if let Some(side) = battle.sides.get_mut(target_pos.0)
                {
                    if let Some(pokemon) = side.pokemon.get_mut(target_pos.1) {
                        let old_hp = pokemon.hp;
                        pokemon.hp = (pokemon.hp + heal_amount).min(pokemon.maxhp);
                        let healed = pokemon.hp - old_hp;
                        if healed > 0 {
                            (pokemon.hp, pokemon.maxhp)
                        } else {
                            (0, 0) // No healing occurred
                        }
                    } else {
                        (0, 0)
                    }
                } else {
                    (0, 0)
                };

                // Log healing after releasing mutable borrow
                if current_hp > 0 {
                    battle.add(
                        "-heal",
                        &[
                            format!("p{}a", target_pos.0 + 1).into(),
                            format!("{}/{}", current_hp, max_hp).into(),
                        ],
                    );
                }
            }
        }

        // Apply status
        // JavaScript: if (moveData.status) { target.setStatus(moveData.status, source, move); }
        if let Some(ref status) = move_data.status {
            eprintln!("[RUN_MOVE_EFFECTS] Applying status '{}' to target {:?}", status, target_pos);
            let status_id = crate::dex_data::ID::new(status);
            let _applied = Pokemon::set_status(battle, target_pos, status_id, Some(_source_pos), None, false);
        }

        // Apply volatile status
        // JavaScript (battle-actions.ts): if (moveData.volatileStatus) { target.addVolatile(moveData.volatileStatus, source, move); }
        if let Some(ref volatile_status) = move_data.volatile_status {
            eprintln!("[RUN_MOVE_EFFECTS] Applying volatile status '{}' to target {:?}", volatile_status, target_pos);
            let volatile_id = crate::dex_data::ID::new(volatile_status);
            Pokemon::add_volatile(battle, target_pos, volatile_id, Some(_source_pos), None, None, move_data.condition.as_ref());
        }

        // Apply side condition
        // JavaScript (battle-actions.ts:1257): if (moveData.sideCondition) {
        //     hitResult = target.side.addSideCondition(moveData.sideCondition, source, move);
        //     didSomething = this.combineResults(didSomething, hitResult);
        // }
        if let Some(ref side_condition) = move_data.side_condition {
            let condition_id = crate::dex_data::ID::new(side_condition);
            battle.add_side_condition(target_pos.0, condition_id, Some(_source_pos), None);
        }

        // Apply weather
        // JavaScript (battle-actions.ts:1263): if (moveData.weather) {
        //     hitResult = this.battle.field.setWeather(moveData.weather, source, move);
        //     didSomething = this.combineResults(didSomething, hitResult);
        // }
        if let Some(ref weather_id) = move_data.weather {
            let weather = crate::dex_data::ID::new(weather_id);
            battle.set_weather(weather, Some(_source_pos), None);
        }

        // Apply terrain
        // JavaScript (battle-actions.ts:1266): if (moveData.terrain) {
        //     hitResult = this.battle.field.setTerrain(moveData.terrain, source, move);
        //     didSomething = this.combineResults(didSomething, hitResult);
        // }
        if let Some(ref terrain_id) = move_data.terrain {
            let terrain = crate::dex_data::ID::new(terrain_id);
            battle.field.set_terrain(terrain, None);
        }

        // Apply pseudo-weather
        // JavaScript (battle-actions.ts:1269): if (moveData.pseudoWeather) {
        //     hitResult = this.battle.field.addPseudoWeather(moveData.pseudoWeather, source, move);
        //     didSomething = this.combineResults(didSomething, hitResult);
        // }
        if let Some(ref pseudo_weather) = move_data.pseudo_weather {
            let pseudo_id = crate::dex_data::ID::new(pseudo_weather);
            battle.field.add_pseudo_weather(pseudo_id, None);
        }

        // Hit events
        // JavaScript (battle-actions.ts:1293-1299):
        // if (moveData.onHit) {
        //     hitResult = this.battle.singleEvent('Hit', moveData, {}, target, source, move);
        //     didSomething = this.combineResults(didSomething, hitResult);
        // }
        // if (!isSelf && !isSecondary) {
        //     this.battle.runEvent('Hit', target, source, move);
        // }

        // Get the move ID for event dispatch
        let move_id = battle.active_move.as_ref().map(|m| m.id.clone()).unwrap_or_else(|| move_data.id.clone());

        // Call singleEvent('Hit', moveData, ...) to trigger move's onHit callback
        // This is what makes King's Shield add the 'stall' volatile
        battle.single_event("Hit", &move_id, Some(target_pos), Some(_source_pos), Some(&move_id));

        // Call runEvent('Hit', ...) to trigger general Hit event handlers
        if !_is_self && !_is_secondary {
            battle.run_event("Hit", Some(target_pos), Some(_source_pos), None, EventResult::Continue, false, false);
        }

        // Keep damage result
        // In the real implementation, we'd check if any effects failed
        // and update result_damages[i] accordingly
    }

    result_damages
}
