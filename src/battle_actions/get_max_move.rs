//! BattleActions::getMaxMove - Get Max Move for a move
//!
//! 1:1 port of getMaxMove from battle-actions.ts

use crate::*;

/// Get Max Move for a given move
/// Equivalent to battle-actions.ts getMaxMove()
///
/// getMaxMove(move: Move, pokemon: Pokemon) {
///     if (typeof move === 'string') move = this.dex.moves.get(move);
///     if (move.name === 'Struggle') return move;
///     if (pokemon.gigantamax && pokemon.canGigantamax && move.category !== 'Status') {
///         const gMaxMove = this.dex.moves.get(pokemon.canGigantamax);
///         if (gMaxMove.exists && gMaxMove.type === move.type) return gMaxMove;
///     }
///     const maxMove = this.dex.moves.get(this.MAX_MOVES[move.category === 'Status' ? move.category : move.type]);
///     if (maxMove.exists) return maxMove;
/// }
pub fn get_max_move<'a>(
    battle: &'a Battle,
    side_index: usize,
    pokemon_index: usize,
    move_id: &str,
) -> Option<&'a crate::dex::MoveData> {
    // if (typeof move === 'string') move = this.dex.moves.get(move);
    let move_data = battle.dex.moves().get(move_id)?;

    // if (move.name === 'Struggle') return move;
    if move_data.name == "Struggle" {
        return Some(move_data);
    }

    // Get pokemon
    let pokemon = battle.sides.get(side_index)
        .and_then(|s| s.pokemon.get(pokemon_index))?;

    // if (pokemon.gigantamax && pokemon.canGigantamax && move.category !== 'Status') {
    if pokemon.gigantamax {
        if let Some(ref can_gigantamax) = pokemon.can_gigantamax {
            if move_data.category != "Status" {
                //     const gMaxMove = this.dex.moves.get(pokemon.canGigantamax);
                //     if (gMaxMove.exists && gMaxMove.type === move.type) return gMaxMove;
                if let Some(gmax_move) = battle.dex.moves().get(can_gigantamax) {
                    if gmax_move.move_type == move_data.move_type {
                        return Some(gmax_move);
                    }
                }
            }
        }
    }

    // const maxMove = this.dex.moves.get(this.MAX_MOVES[move.category === 'Status' ? move.category : move.type]);
    // if (maxMove.exists) return maxMove;
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
            _ => return None,
        }
    };

    battle.dex.moves().get(max_move_name)
}
