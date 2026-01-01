//! BattleActions::runMoveEffects - Run move effects (boosts, healing, status, etc.)
//!
//! 1:1 port of runMoveEffects from battle-actions.ts:1201

use crate::*;

/// Run move effects (boosts, healing, status, etc.)
/// Equivalent to runMoveEffects() in battle-actions.ts:1201
pub fn run_move_effects(
    battle: &mut Battle,
    damages: &[Option<i32>],
    targets: &[Option<(usize, usize)>],
    _source_pos: (usize, usize),
    move_data: &crate::dex::MoveData,
    _is_secondary: bool,
    _is_self: bool,
) -> Vec<Option<i32>> {
    eprintln!("[RUN_MOVE_EFFECTS] Called for move: {:?}, has status: {:?}", move_data.id, move_data.status);
    let result_damages = damages.to_vec();

    for &target in targets.iter() {
        if target.is_none() {
            continue;
        }

        let target_pos = target.unwrap();

        // Apply boosts
        if let Some(ref boosts_map) = move_data.boosts {
            // Convert HashMap<String, i32> to BoostsTable
            let mut boosts_table = crate::dex_data::BoostsTable::default();
            for (stat_name, &value) in boosts_map.iter() {
                match stat_name.to_lowercase().as_str() {
                    "atk" => boosts_table.atk = value as i8,
                    "def" => boosts_table.def = value as i8,
                    "spa" => boosts_table.spa = value as i8,
                    "spd" => boosts_table.spd = value as i8,
                    "spe" => boosts_table.spe = value as i8,
                    "accuracy" => boosts_table.accuracy = value as i8,
                    "evasion" => boosts_table.evasion = value as i8,
                    _ => {}
                }
            }

            // Apply boosts to target
            if let Some(side) = battle.sides.get_mut(target_pos.0) {
                if let Some(pokemon) = side.pokemon.get_mut(target_pos.1) {
                    for boost_id in crate::dex_data::BoostID::all() {
                        let change = boosts_table.get(*boost_id);
                        if change != 0 {
                            pokemon.boosts.boost(*boost_id, change);
                        }
                    }
                }
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
        if let Some(ref status) = move_data.status {
            eprintln!("[RUN_MOVE_EFFECTS] Applying status '{}' to target {:?}", status, target_pos);
            if let Some(side) = battle.sides.get_mut(target_pos.0) {
                if let Some(pokemon) = side.pokemon.get_mut(target_pos.1) {
                    eprintln!("[RUN_MOVE_EFFECTS] Target {} current status: '{}'", pokemon.name, pokemon.status);
                    // Simple status application (full version would check immunity)
                    if pokemon.status.is_empty() {
                        pokemon.status = crate::dex_data::ID::new(status);
                        eprintln!("[RUN_MOVE_EFFECTS] Applied status '{}' to {}", status, pokemon.name);
                        battle.add("-status", &[format!("p{}a", target_pos.0 + 1).into(), status.as_str().into()]);
                    } else {
                        eprintln!("[RUN_MOVE_EFFECTS] Status not applied - {} already has status '{}'", pokemon.name, pokemon.status);
                    }
                }
            }
        }

        // Apply volatile status
        // JavaScript (battle-actions.ts): if (moveData.volatileStatus) { target.addVolatile(moveData.volatileStatus, source, move); }
        if let Some(ref volatile_status) = move_data.volatile_status {
            eprintln!("[RUN_MOVE_EFFECTS] Applying volatile status '{}' to target {:?}", volatile_status, target_pos);

            let volatile_id = crate::dex_data::ID::new(volatile_status);

            // Check if pokemon already has this volatile
            let already_has_volatile = {
                if let Some(side) = battle.sides.get(target_pos.0) {
                    if let Some(pokemon) = side.pokemon.get(target_pos.1) {
                        pokemon.volatiles.contains_key(&volatile_id)
                    } else {
                        true // Pokemon doesn't exist, treat as "already has"
                    }
                } else {
                    true // Side doesn't exist
                }
            };

            if !already_has_volatile {
                // Get default duration from move's condition first, then from dex.conditions
                // JS: if (status.duration) this.volatiles[status.id].duration = status.duration;
                // When adding a volatile via move.volatileStatus, the duration comes from move.condition.duration
                let move_condition_duration = move_data.condition.as_ref().and_then(|c| c.duration);
                let dex_condition_duration = battle.dex.conditions.get(&volatile_id)
                    .and_then(|cond| cond.duration);
                let default_duration = move_condition_duration.or(dex_condition_duration);

                // Check if condition has a durationCallback
                // JS: if (status.durationCallback) {
                //     this.volatiles[status.id].duration = status.durationCallback.call(this.battle, this, source, sourceEffect);
                // }
                let callback_duration = crate::data::duration_callbacks::dispatch_duration_callback(
                    battle,
                    volatile_id.as_str(),
                    target_pos,
                    Some(_source_pos),
                );

                // durationCallback overrides default duration
                let final_duration = callback_duration.or(default_duration);

                // Add the volatile
                if let Some(side) = battle.sides.get_mut(target_pos.0) {
                    if let Some(pokemon) = side.pokemon.get_mut(target_pos.1) {
                        let mut state = crate::event_system::EffectState::new(volatile_id.clone());
                        state.duration = final_duration;
                        state.source_slot = Some(_source_pos.1);

                        pokemon.volatiles.insert(volatile_id.clone(), state);
                        eprintln!("[RUN_MOVE_EFFECTS] Successfully added volatile '{}' with duration {:?}", volatile_status, final_duration);
                    }
                }
            } else {
                eprintln!("[RUN_MOVE_EFFECTS] Volatile '{}' already exists on target", volatile_status);
            }

            // Verify volatile was actually added to battle state
            eprintln!("[RUN_MOVE_EFFECTS] VERIFICATION: Checking battle.sides[{}].pokemon[{}] volatiles...", target_pos.0, target_pos.1);
            if let Some(side) = battle.sides.get(target_pos.0) {
                if let Some(pokemon) = side.pokemon.get(target_pos.1) {
                    eprintln!("[RUN_MOVE_EFFECTS] VERIFICATION: {} now has {} volatiles: {:?}",
                        pokemon.name, pokemon.volatiles.len(), pokemon.volatiles.keys().collect::<Vec<_>>());
                }
            }

            // Trigger Start event for the volatile
            // JavaScript: this.battle.singleEvent('Start', status, this.volatiles[status.id], this, source, sourceEffect);
            let volatile_id_obj = crate::dex_data::ID::new(volatile_status);
            battle.single_event("Start", &volatile_id_obj, Some(target_pos), Some(_source_pos), None);
        }

        // Keep damage result
        // In the real implementation, we'd check if any effects failed
        // and update result_damages[i] accordingly
    }

    result_damages
}
