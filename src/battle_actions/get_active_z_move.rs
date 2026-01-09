//! BattleActions::getActiveZMove - Get active Z-Move from a move
//!
//! 1:1 port of getActiveZMove from battle-actions.ts

use crate::*;
use crate::battle_actions::ActiveMove;

/// Get active Z-Move for a given move
/// Equivalent to battle-actions.ts getActiveZMove()
///
/// getActiveZMove(move: Move, pokemon: Pokemon): ActiveMove {
///     if (pokemon) {
///         const item = pokemon.getItem();
///         if (move.name === item.zMoveFrom) {
///             const zMove = this.dex.getActiveMove(item.zMove as string);
///             zMove.isZOrMaxPowered = true;
///             return zMove;
///         }
///     }
///
///     if (move.category === 'Status') {
///         const zMove = this.dex.getActiveMove(move);
///         zMove.isZ = true;
///         zMove.isZOrMaxPowered = true;
///         return zMove;
///     }
///     const zMove = this.dex.getActiveMove(this.Z_MOVES[move.type]);
///     zMove.basePower = move.zMove!.basePower!;
///     zMove.category = move.category;
///     // copy the priority for Quick Guard
///     zMove.priority = move.priority;
///     zMove.isZOrMaxPowered = true;
///     return zMove;
/// }
// TODO: Verify move parameter type matches JavaScript's ActiveMove usage
pub fn get_active_z_move(
    battle: &Battle,
    side_index: usize,
    pokemon_index: usize,
    move_id: &str,
) -> ActiveMove {
    // Get the base move
    let move_data = battle.dex.moves().get(move_id).expect("Move not found");

    // Get pokemon
    let pokemon = battle.sides.get(side_index)
        .and_then(|s| s.pokemon.get(pokemon_index));

    //     if (pokemon) {
    //         const item = pokemon.getItem();
    //         if (move.name === item.zMoveFrom) {
    //             const zMove = this.dex.getActiveMove(item.zMove as string);
    //             zMove.isZOrMaxPowered = true;
    //             return zMove;
    //         }
    //     }
    if let Some(pokemon) = pokemon {
        if let Some(item) = battle.dex.items().get(&pokemon.item.as_str()) {
            if let Some(z_move_from) = item.extra.get("zMoveFrom").and_then(|v| v.as_str()) {
                if move_data.name == z_move_from {
                    // Get the Z-move name from item.zMove
                    if let Some(ref z_move_value) = item.z_move {
                        if let Some(z_move_name) = z_move_value.as_str() {
                            // Get the active move data for the Z-move
                            if let Some(z_move_data) = battle.dex.moves().get(z_move_name) {
                                // TODO: Implement full getActiveMove conversion
                                // For now, create a basic ActiveMove with z-move properties
                                let mut active_move = move_data_to_active_move(z_move_data);
                                active_move.is_z_or_max_powered = true;
                                return active_move;
                            }
                        }
                    }
                }
            }
        }
    }

    //     if (move.category === 'Status') {
    //         const zMove = this.dex.getActiveMove(move);
    //         zMove.isZ = true;
    //         zMove.isZOrMaxPowered = true;
    //         return zMove;
    //     }
    if move_data.category == "Status" {
        let mut active_move = move_data_to_active_move(move_data);
        active_move.is_z = Some("true".to_string());
        active_move.is_z_or_max_powered = true;
        return active_move;
    }

    //     const zMove = this.dex.getActiveMove(this.Z_MOVES[move.type]);
    //     zMove.basePower = move.zMove!.basePower!;
    //     zMove.category = move.category;
    //     // copy the priority for Quick Guard
    //     zMove.priority = move.priority;
    //     zMove.isZOrMaxPowered = true;
    //     return zMove;
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
        _ => panic!("Unknown type for Z-move: {}", move_data.move_type),
    };

    let z_move_data = battle.dex.moves().get(z_move_name).expect("Z-move not found");
    let mut active_move = move_data_to_active_move(z_move_data);

    // Set base power from original move's zMove data
    if let Some(ref z_move_info) = move_data.z_move {
        if let Some(base_power) = z_move_info.get("basePower").and_then(|v| v.as_i64()) {
            active_move.base_power = base_power as i32;
        }
    }

    active_move.category = move_data.category.clone();
    active_move.priority = move_data.priority;
    active_move.is_z_or_max_powered = true;

    active_move
}

/// Helper function to convert MoveData to ActiveMove
/// TODO: This should be a proper Dex method (getActiveMove)
fn move_data_to_active_move(move_data: &crate::dex::MoveData) -> ActiveMove {
    // Create a basic ActiveMove from MoveData
    // This is a simplified conversion - full implementation would be in dex.getActiveMove()

    ActiveMove {
        id: move_data.id.clone(),
        name: move_data.name.clone(),
        fullname: format!("move: {}", move_data.name),
        num: move_data.num,
        exists: true,
        gen: 9, // Default generation
        short_desc: String::new(), // MoveData doesn't have this
        desc: String::new(), // MoveData doesn't have this
        is_nonstandard: None, // MoveData doesn't have this
        duration: None,
        no_copy: false,
        affects_fainted: false,
        source_effect_name: None,
        condition: None,
        base_power: move_data.base_power,
        accuracy: move_data.accuracy.clone(),
        pp: move_data.pp, // Already u8
        category: move_data.category.clone(),
        move_type: move_data.move_type.clone(),
        priority: move_data.priority,
        target: move_data.target.clone(),
        flags: move_data.flags.clone(),
        ..Default::default()
    }
}
