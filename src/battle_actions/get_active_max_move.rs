//! BattleActions::getActiveMaxMove - Get active Max Move from a move
//!
//! 1:1 port of getActiveMaxMove from battle-actions.ts

use crate::*;
use crate::battle_actions::ActiveMove;
use crate::dex::MoveData;

/// Get active Max Move for a given move
/// Equivalent to battle-actions.ts getActiveMaxMove()
///
/// getActiveMaxMove(move: Move, pokemon: Pokemon) {
///     if (typeof move === 'string') move = this.dex.getActiveMove(move);
///     if (move.name === 'Struggle') return this.dex.getActiveMove(move);
///     let maxMove = this.dex.getActiveMove(this.MAX_MOVES[move.category === 'Status' ? move.category : move.type]);
///     if (move.category !== 'Status') {
///         if (pokemon.gigantamax && pokemon.canGigantamax) {
///             const gMaxMove = this.dex.getActiveMove(pokemon.canGigantamax);
///             if (gMaxMove.exists && gMaxMove.type === move.type) maxMove = gMaxMove;
///         }
///         if (!move.maxMove?.basePower) throw new Error(`${move.name} doesn't have a maxMove basePower`);
///         if (!['gmaxdrumsolo', 'gmaxfireball', 'gmaxhydrosnipe'].includes(maxMove.id)) {
///             maxMove.basePower = move.maxMove.basePower;
///         }
///         maxMove.category = move.category;
///     }
///     maxMove.baseMove = move.id;
///     // copy the priority for Psychic Terrain, Quick Guard
///     maxMove.priority = move.priority;
///     maxMove.isZOrMaxPowered = true;
///     return maxMove;
/// }
pub fn get_active_max_move(
    battle: &Battle,
    side_index: usize,
    pokemon_index: usize,
    move_data: &MoveData,
) -> ActiveMove {
    // if (move.name === 'Struggle') return this.dex.getActiveMove(move);
    if move_data.name == "Struggle" {
        return move_data_to_active_move(move_data);
    }

    // Get pokemon
    let pokemon = battle.sides.get(side_index)
        .and_then(|s| s.pokemon.get(pokemon_index))
        .expect("Pokemon not found");

    // let maxMove = this.dex.getActiveMove(this.MAX_MOVES[move.category === 'Status' ? move.category : move.type]);
    let max_move_name = if move_data.category == "Status" {
        "Max Guard"
    } else {
        match move_data.move_type.as_str() {
            "Flying" => "Max Airstream",
            "Dark" => "Max Darkness",
            "Fire" => "Max Flare",
            "Bug" => "Max Flutterby",
            "Water" => "Max Geyser",
            "Ice" => "Max Hailstorm",
            "Fighting" => "Max Knuckle",
            "Electric" => "Max Lightning",
            "Psychic" => "Max Mindstorm",
            "Poison" => "Max Ooze",
            "Grass" => "Max Overgrowth",
            "Ghost" => "Max Phantasm",
            "Ground" => "Max Quake",
            "Rock" => "Max Rockfall",
            "Fairy" => "Max Starfall",
            "Steel" => "Max Steelspike",
            "Normal" => "Max Strike",
            "Dragon" => "Max Wyrmwind",
            _ => panic!("Unknown type for Max move: {}", move_data.move_type),
        }
    };

    let max_move_data = battle.dex.moves().get(max_move_name).expect("Max move not found");
    let mut max_move = move_data_to_active_move(max_move_data);

    // if (move.category !== 'Status') {
    if move_data.category != "Status" {
        //     if (pokemon.gigantamax && pokemon.canGigantamax) {
        //         const gMaxMove = this.dex.getActiveMove(pokemon.canGigantamax);
        //         if (gMaxMove.exists && gMaxMove.type === move.type) maxMove = gMaxMove;
        //     }
        if pokemon.gigantamax {
            if let Some(ref can_gigantamax) = pokemon.can_gigantamax {
                if let Some(gmax_move_data) = battle.dex.moves().get(can_gigantamax) {
                    if gmax_move_data.move_type == move_data.move_type {
                        max_move = move_data_to_active_move(gmax_move_data);
                    }
                }
            }
        }

        //     if (!move.maxMove?.basePower) throw new Error(`${move.name} doesn't have a maxMove basePower`);
        let max_move_base_power = move_data.max_move.as_ref()
            .map(|mm| mm.base_power)
            .expect(&format!("{} doesn't have a maxMove basePower", move_data.name));

        //     if (!['gmaxdrumsolo', 'gmaxfireball', 'gmaxhydrosnipe'].includes(maxMove.id)) {
        //         maxMove.basePower = move.maxMove.basePower;
        //     }
        let max_move_id_str = max_move.id.as_str();
        if max_move_id_str != "gmaxdrumsolo" &&
           max_move_id_str != "gmaxfireball" &&
           max_move_id_str != "gmaxhydrosnipe" {
            max_move.base_power = max_move_base_power;
        }

        //     maxMove.category = move.category;
        max_move.category = move_data.category.clone();
    }

    // maxMove.baseMove = move.id;
    max_move.base_move = Some(move_data.id.clone());

    // // copy the priority for Psychic Terrain, Quick Guard
    // maxMove.priority = move.priority;
    max_move.priority = move_data.priority;

    // maxMove.isZOrMaxPowered = true;
    max_move.is_z_or_max_powered = true;

    // return maxMove;
    max_move
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
        pp: move_data.pp,
        category: move_data.category.clone(),
        move_type: move_data.move_type.clone(),
        priority: move_data.priority,
        target: move_data.target.clone(),
        flags: move_data.flags.clone(),
        ..Default::default()
    }
}
