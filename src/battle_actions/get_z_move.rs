//! BattleActions::getZMove - Get Z-Move for a move
//!
//! 1:1 port of getZMove from battle-actions.ts

use crate::*;

/// Get Z-Move for a given move
/// Equivalent to battle-actions.ts getZMove()
///
/// getZMove(move: Move, pokemon: Pokemon, skipChecks?: boolean): string | undefined {
///     const item = pokemon.getItem();
///     if (!skipChecks) {
///         if (pokemon.side.zMoveUsed) return;
///         if (!item.zMove) return;
///         if (item.itemUser && !item.itemUser.includes(pokemon.species.name)) return;
///         const moveData = pokemon.getMoveData(move);
///         // Draining the PP of the base move prevents the corresponding Z-move from being used.
///         if (!moveData?.pp) return;
///     }
///
///     if (item.zMoveFrom) {
///         if (move.name === item.zMoveFrom) return item.zMove as string;
///     } else if (item.zMove === true) {
///         if (move.type === item.zMoveType) {
///             if (move.category === "Status") {
///                 return move.name;
///             } else if (move.zMove?.basePower) {
///                 return this.Z_MOVES[move.type];
///             }
///         }
///     }
/// }
pub fn get_z_move(
    battle: &Battle,
    side_index: usize,
    pokemon_index: usize,
    move_id: &str,
    skip_checks: bool,
) -> Option<String> {
    // Get pokemon
    let pokemon = match battle.sides.get(side_index)
        .and_then(|s| s.pokemon.get(pokemon_index)) {
        Some(p) => p,
        None => return None,
    };

    // const item = pokemon.getItem();
    let item = battle.dex.items().get(pokemon.item.as_str())?;

    // if (!skipChecks) {
    if !skip_checks {
        //     if (pokemon.side.zMoveUsed) return;
        if battle.sides[side_index].z_move_used {
            return None;
        }

        //     if (!item.zMove) return;
        if item.z_move.is_none() {
            return None;
        }

        //     if (item.itemUser && !item.itemUser.includes(pokemon.species.name)) return;
        if let Some(ref item_users) = item.item_user {
            let species = battle.dex.species().get(&pokemon.base_species.as_str())?;
            if !item_users.contains(&species.name) {
                return None;
            }
        }

        //     const moveData = pokemon.getMoveData(move);
        //     // Draining the PP of the base move prevents the corresponding Z-move from being used.
        //     if (!moveData?.pp) return;
        // Check if the Pokemon has a move slot with PP for this move
        let has_pp = pokemon.move_slots.iter().any(|slot| {
            slot.id.as_str() == move_id && slot.pp > 0
        });
        if !has_pp {
            return None;
        }
    }

    // Get move data
    let move_data = battle.dex.moves().get(move_id)?;

    //     if (item.zMoveFrom) {
    //         if (move.name === item.zMoveFrom) return item.zMove as string;
    //     }
    // Access zMoveFrom from extra field
    if let Some(z_move_from) = item.extra.get("zMoveFrom").and_then(|v| v.as_str()) {
        if move_data.name == z_move_from {
            // item.zMove should be a string in this case
            if let Some(ref z_move_value) = item.z_move {
                if let Some(z_move_str) = z_move_value.as_str() {
                    return Some(z_move_str.to_string());
                }
            }
        }
    }
    // else if (item.zMove === true) {
    else if let Some(ref z_move_value) = item.z_move {
        if z_move_value.as_bool() == Some(true) {
            //     if (move.type === item.zMoveType) {
            if let Some(z_move_type) = item.extra.get("zMoveType").and_then(|v| v.as_str()) {
                if move_data.move_type == z_move_type {
                    //         if (move.category === "Status") {
                    //             return move.name;
                    //         }
                    if move_data.category == "Status" {
                        return Some(move_data.name.clone());
                    }
                    //         else if (move.zMove?.basePower) {
                    //             return this.Z_MOVES[move.type];
                    //         }
                    else if move_data.z_move.as_ref()
                        .and_then(|zm| zm.get("basePower"))
                        .is_some()
                    {
                        // Use the Z_MOVES constant to get the Z-move name
                        let z_move_name = match move_data.move_type.as_str() {
                            "Poison" => "Acid Downpour",
                            "Fighting" => "All-Out Pummeling",
                            "Dark" => "Black Hole Eclipse",
                            "Grass" => "Bloom Doom",
                            "Normal" => "Breakneck Blitz",
                            "Rock" => "Continental Crush",
                            "Steel" => "Corkscrew Crash",
                            "Dragon" => "Devastating Drake",
                            "Electric" => "Gigavolt Havoc",
                            "Water" => "Hydro Vortex",
                            "Fire" => "Inferno Overdrive",
                            "Ghost" => "Never-Ending Nightmare",
                            "Bug" => "Savage Spin-Out",
                            "Psychic" => "Shattered Psyche",
                            "Ice" => "Subzero Slammer",
                            "Flying" => "Supersonic Skystrike",
                            "Ground" => "Tectonic Rage",
                            "Fairy" => "Twinkle Tackle",
                            _ => return None,
                        };
                        return Some(z_move_name.to_string());
                    }
                }
            }
        }
    }

    None
}
