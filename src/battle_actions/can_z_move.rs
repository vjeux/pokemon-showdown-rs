use crate::*;
use crate::battle_actions::CanZMoveParams;

impl<'a> BattleActions<'a> {

    /// Check if Pokemon can use Z-Move
    /// Equivalent to canZMove in battle-actions.ts
    //
    // 	canZMove(pokemon: Pokemon) {
    // 		if (pokemon.side.zMoveUsed ||
    // 			(pokemon.transformed &&
    // 				(pokemon.species.isMega || pokemon.species.isPrimal || pokemon.species.forme === "Ultra"))
    // 		) return;
    // 		const item = pokemon.getItem();
    // 		if (!item.zMove) return;
    // 		if (item.itemUser && !item.itemUser.includes(pokemon.species.name)) return;
    // 		let atLeastOne = false;
    // 		let mustStruggle = true;
    // 		const zMoves: ZMoveOptions = [];
    // 		for (const moveSlot of pokemon.moveSlots) {
    // 			if (moveSlot.pp <= 0) {
    // 				zMoves.push(null);
    // 				continue;
    // 			}
    // 			if (!moveSlot.disabled) {
    // 				mustStruggle = false;
    // 			}
    // 			const move = this.dex.moves.get(moveSlot.move);
    // 			let zMoveName = this.getZMove(move, pokemon, true) || '';
    // 			if (zMoveName) {
    // 				const zMove = this.dex.moves.get(zMoveName);
    // 				if (!zMove.isZ && zMove.category === 'Status') zMoveName = "Z-" + zMoveName;
    // 				zMoves.push({ move: zMoveName, target: zMove.target });
    // 			} else {
    // 				zMoves.push(null);
    // 			}
    // 			if (zMoveName) atLeastOne = true;
    // 		}
    // 		if (atLeastOne && !mustStruggle) return zMoves;
    // 	}
    //
    pub fn can_z_move(params: CanZMoveParams) -> bool {
        if params.z_move_used {
            return false;
        }
        if params.is_transformed
            && (params.species_is_mega
                || params.species_is_primal
                || params.species_forme == "Ultra")
        {
            return false;
        }
        if !params.item_z_move {
            return false;
        }
        if let Some(users) = params.item_user {
            if !users.iter().any(|u| u == params.species_name) {
                return false;
            }
        }
        true
    }
}
