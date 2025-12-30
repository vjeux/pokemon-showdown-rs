use crate::*;
use crate::battle_actions::ZMoveParams;
use super::get_z_move_name;

impl<'a> BattleActions<'a> {

    // =========================================================================
    // Z-MOVE METHODS
    // =========================================================================

    /// Get Z-Move for a move
    /// Equivalent to getZMove in battle-actions.ts
    //
    // 	getZMove(move: Move, pokemon: Pokemon, skipChecks?: boolean): string | undefined {
    // 		const item = pokemon.getItem();
    // 		if (!skipChecks) {
    // 			if (pokemon.side.zMoveUsed) return;
    // 			if (!item.zMove) return;
    // 			if (item.itemUser && !item.itemUser.includes(pokemon.species.name)) return;
    // 			const moveData = pokemon.getMoveData(move);
    // 			// Draining the PP of the base move prevents the corresponding Z-move from being used.
    // 			if (!moveData?.pp) return;
    // 		}
    //
    // 		if (item.zMoveFrom) {
    // 			if (move.name === item.zMoveFrom) return item.zMove as string;
    // 		} else if (item.zMove === true) {
    // 			if (move.type === item.zMoveType) {
    // 				if (move.category === "Status") {
    // 					return move.name;
    // 				} else if (move.zMove?.basePower) {
    // 					return this.Z_MOVES[move.type];
    // 				}
    // 			}
    // 		}
    // 	}
    //
    pub fn get_z_move(params: ZMoveParams) -> Option<String> {
        if params.z_move_used {
            return None;
        }

        // Check for signature Z-move
        if let Some(z_move_from) = params.item_z_move_from {
            if params.move_name == z_move_from {
                return params.item_z_move.map(|s| s.to_string());
            }
        }

        // Check for type-based Z-move
        if params.item_z_move.is_some() {
            if let Some(z_type) = params.item_z_move_type {
                if params.move_type == z_type {
                    if params.move_category == "Status" {
                        return Some(params.move_name.to_string());
                    } else if params.z_move_base_power.is_some() {
                        return Some(get_z_move_name(params.move_type).to_string());
                    }
                }
            }
        }

        None
    }
}
