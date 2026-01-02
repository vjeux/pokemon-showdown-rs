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
        // JS: if (!skipChecks) {
        // Note: Parameter is can_dynamax, JavaScript has skipChecks (inverted logic)

        // JS:     if (!this.side.canDynamaxNow()) return;
        // Note: Missing side.canDynamaxNow() check (needs Side reference)

        // JS:     if (
        // JS:         this.species.isMega || this.species.isPrimal || this.species.forme === "Ultra" ||
        // JS:         this.getItem().zMove || this.canMegaEvo
        // JS:     ) {
        // JS:         return;
        // JS:     }
        // Note: Missing species checks for Mega, Primal, Ultra, Z-Move, Mega Evo
        // Note: Would need species data and item data access

        // JS:     if (this.species.cannotDynamax || this.illusion?.species.cannotDynamax) return;
        // Note: Missing cannotDynamax check (Zacian, Zamazenta, Eternatus)
        // Note: Missing illusion species check

        // JS: }
        // JS: const result: DynamaxOptions = { maxMoves: [] };
        // Note: Should always build maxMoves array

        // JS: let atLeastOne = false;
        // Note: Missing atLeastOne tracking to return None if all moves disabled

        // JS: for (const moveSlot of this.moveSlots) {
        // JS:     const move = this.battle.dex.moves.get(moveSlot.id);
        // Note: Should loop through move slots

        // JS:     const maxMove = this.battle.actions.getMaxMove(move, this);
        // Note: Missing getMaxMove() call (converts normal move to Max Move)

        // JS:     if (maxMove) {
        // JS:         if (this.maxMoveDisabled(move)) {
        // JS:             result.maxMoves.push({ move: maxMove.id, target: maxMove.target, disabled: true });
        // JS:         } else {
        // JS:             result.maxMoves.push({ move: maxMove.id, target: maxMove.target });
        // JS:             atLeastOne = true;
        // JS:         }
        // JS:     }
        // JS: }
        // Note: Missing max move generation for each move slot
        // Note: Missing maxMoveDisabled() check per move

        // JS: if (!atLeastOne) return;
        // Note: Should return None if all max moves are disabled

        // JS: if (this.canGigantamax && this.gigantamax) result.gigantamax = this.canGigantamax;
        // Note: Missing Gigantamax field (species name like "Charizard")

        // JS: return result;
        if !can_dynamax || self.has_volatile(&ID::new("dynamax")) {
            return None;
        }

        Some(serde_json::json!({
            "canDynamax": true
        }))
    }
}
