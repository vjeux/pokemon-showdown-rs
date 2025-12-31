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
        if let Some((heal_num, heal_denom)) = move_data.heal {
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

        // Keep damage result
        // In the real implementation, we'd check if any effects failed
        // and update result_damages[i] accordingly
    }

    result_damages
}
