use crate::*;

impl Pokemon {

    /// Get Dynamax request data
    /// Equivalent to getDynamaxRequest in pokemon.ts
    //
    // 	getDynamaxRequest(skipChecks?: boolean) {
    // 		// {gigantamax?: string, maxMoves: {[k: string]: string} | null}[]
    // 		if (!skipChecks) {
    // 			if (!this.side.canDynamaxNow()) return;
    // 			if (
    // 				this.species.isMega || this.species.isPrimal || this.species.forme === "Ultra" ||
    // 				this.getItem().zMove || this.canMegaEvo
    // 			) {
    // 				return;
    // 			}
    // 			// Some pokemon species are unable to dynamax
    // 			if (this.species.cannotDynamax || this.illusion?.species.cannotDynamax) return;
    // 		}
    // 		const result: DynamaxOptions = { maxMoves: [] };
    // 		let atLeastOne = false;
    // 		for (const moveSlot of this.moveSlots) {
    // 			const move = this.battle.dex.moves.get(moveSlot.id);
    // 			const maxMove = this.battle.actions.getMaxMove(move, this);
    // 			if (maxMove) {
    // 				if (this.maxMoveDisabled(move)) {
    // 					result.maxMoves.push({ move: maxMove.id, target: maxMove.target, disabled: true });
    // 				} else {
    // 					result.maxMoves.push({ move: maxMove.id, target: maxMove.target });
    // 					atLeastOne = true;
    // 				}
    // 			}
    // 		}
    // 		if (!atLeastOne) return;
    // 		if (this.canGigantamax && this.gigantamax) result.gigantamax = this.canGigantamax;
    // 		return result;
    // 	}
    //
    pub fn get_dynamax_request(&self, can_dynamax: bool) -> Option<serde_json::Value> {
        // TODO: implement the same logic as JavaScript
        
        if !can_dynamax || self.has_volatile(&ID::new("dynamax")) {
            return None;
        }

        Some(serde_json::json!({
            "canDynamax": true
        }))
    }
}
