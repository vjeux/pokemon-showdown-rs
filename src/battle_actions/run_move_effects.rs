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
            Pokemon::add_volatile(battle, target_pos, volatile_id, Some(_source_pos), None, None);
        }

        // Keep damage result
        // In the real implementation, we'd check if any effects failed
        // and update result_damages[i] accordingly
    }

    result_damages
}
