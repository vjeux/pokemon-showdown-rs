//! BattleActions::canZMove - Check if Pokemon can use Z-Moves
//!
//! 1:1 port of canZMove from battle-actions.ts

use crate::*;
use crate::battle_actions::{ZMoveOption, get_z_move};

/// Check if Pokemon can use Z-Moves and return available Z-moves
/// Equivalent to battle-actions.ts canZMove()
///
/// canZMove(pokemon: Pokemon) {
///     if (pokemon.side.zMoveUsed ||
///         (pokemon.transformed &&
///             (pokemon.species.isMega || pokemon.species.isPrimal || pokemon.species.forme === "Ultra"))
///     ) return;
///     const item = pokemon.getItem();
///     if (!item.zMove) return;
///     if (item.itemUser && !item.itemUser.includes(pokemon.species.name)) return;
///     let atLeastOne = false;
///     let mustStruggle = true;
///     const zMoves: ZMoveOptions = [];
///     for (const moveSlot of pokemon.moveSlots) {
///         if (moveSlot.pp <= 0) {
///             zMoves.push(null);
///             continue;
///         }
///         if (!moveSlot.disabled) {
///             mustStruggle = false;
///         }
///         const move = this.dex.moves.get(moveSlot.move);
///         let zMoveName = this.getZMove(move, pokemon, true) || '';
///         if (zMoveName) {
///             const zMove = this.dex.moves.get(zMoveName);
///             if (!zMove.isZ && zMove.category === 'Status') zMoveName = "Z-" + zMoveName;
///             zMoves.push({ move: zMoveName, target: zMove.target });
///         } else {
///             zMoves.push(null);
///         }
///         if (zMoveName) atLeastOne = true;
///     }
///     if (atLeastOne && !mustStruggle) return zMoves;
/// }
pub fn can_z_move(
    battle: &Battle,
    side_index: usize,
    pokemon_index: usize,
) -> Option<Vec<Option<ZMoveOption>>> {
    // Get pokemon
    let pokemon = match battle.sides.get(side_index)
        .and_then(|s| s.pokemon.get(pokemon_index)) {
        Some(p) => p,
        None => return None,
    };

    //     if (pokemon.side.zMoveUsed ||
    //         (pokemon.transformed &&
    //             (pokemon.species.isMega || pokemon.species.isPrimal || pokemon.species.forme === "Ultra"))
    //     ) return;
    if battle.sides[side_index].z_move_used {
        return None;
    }

    if pokemon.transformed {
        let species = battle.dex.species().get(&pokemon.base_species.as_str())?;

        // Check if species is Mega, Primal, or Ultra forme
        if species.is_mega {
            return None;
        }

        // Check for isPrimal - need to check species name patterns
        if species.name.contains("Primal") {
            return None;
        }

        // Check for Ultra forme
        if species.forme.as_ref().map(|f| f == "Ultra").unwrap_or(false) {
            return None;
        }
    }

    //     const item = pokemon.getItem();
    //     if (!item.zMove) return;
    let item = battle.dex.items().get(pokemon.item.as_str())?;
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

    //     let atLeastOne = false;
    //     let mustStruggle = true;
    //     const zMoves: ZMoveOptions = [];
    let mut at_least_one = false;
    let mut must_struggle = true;
    let mut z_moves: Vec<Option<ZMoveOption>> = Vec::new();

    //     for (const moveSlot of pokemon.moveSlots) {
    for move_slot in &pokemon.move_slots {
        //         if (moveSlot.pp <= 0) {
        //             zMoves.push(null);
        //             continue;
        //         }
        if move_slot.pp == 0 {
            z_moves.push(None);
            continue;
        }

        //         if (!moveSlot.disabled) {
        //             mustStruggle = false;
        //         }
        if !move_slot.disabled {
            must_struggle = false;
        }

        //         const move = this.dex.moves.get(moveSlot.move);
        //         let zMoveName = this.getZMove(move, pokemon, true) || '';
        let move_data = match battle.dex.moves().get(move_slot.id.as_str()) {
            Some(m) => m,
            None => {
                z_moves.push(None);
                continue;
            }
        };
        let z_move_name = get_z_move(battle, side_index, pokemon_index, move_data, true);

        //         if (zMoveName) {
        if let Some(mut z_move_name) = z_move_name {
            //             const zMove = this.dex.moves.get(zMoveName);
            //             if (!zMove.isZ && zMove.category === 'Status') zMoveName = "Z-" + zMoveName;
            if let Some(z_move_data) = battle.dex.moves().get(&z_move_name) {
                if z_move_data.is_z.is_none() && z_move_data.category == "Status" {
                    z_move_name = format!("Z-{}", z_move_name);
                }

                //             zMoves.push({ move: zMoveName, target: zMove.target });
                z_moves.push(Some(ZMoveOption {
                    move_name: z_move_name.clone(),
                    target: z_move_data.target.clone(),
                }));
            } else {
                z_moves.push(None);
            }

            //         if (zMoveName) atLeastOne = true;
            at_least_one = true;
        } else {
            //         } else {
            //             zMoves.push(null);
            //         }
            z_moves.push(None);
        }
    }

    //     if (atLeastOne && !mustStruggle) return zMoves;
    if at_least_one && !must_struggle {
        Some(z_moves)
    } else {
        None
    }
}
