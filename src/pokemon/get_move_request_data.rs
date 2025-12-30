use crate::*;

impl Pokemon {

    /// Get move request data for protocol
    /// Equivalent to getMoveRequestData in pokemon.ts
    //
    // 	getMoveRequestData() {
    // 		let lockedMove = this.maybeLocked ? null : this.getLockedMove();
    //
    // 		// Information should be restricted for the last active Pokémon
    // 		const isLastActive = this.isLastActive();
    // 		const canSwitchIn = this.battle.canSwitch(this.side) > 0;
    // 		let moves = this.getMoves(lockedMove, isLastActive);
    //
    // 		if (!moves.length) {
    // 			moves = [{ move: 'Struggle', id: 'struggle' as ID, target: 'randomNormal', disabled: false }];
    // 			lockedMove = 'struggle' as ID;
    // 		}
    //
    // 		const data: PokemonMoveRequestData = {
    // 			moves,
    // 		};
    //
    // 		if (isLastActive) {
    // 			this.maybeDisabled = this.maybeDisabled && !lockedMove;
    // 			this.maybeLocked = this.maybeLocked || this.maybeDisabled;
    // 			if (this.maybeDisabled) {
    // 				data.maybeDisabled = this.maybeDisabled;
    // 			}
    // 			if (this.maybeLocked) {
    // 				data.maybeLocked = this.maybeLocked;
    // 			}
    // 			if (canSwitchIn) {
    // 				if (this.trapped === true) {
    // 					data.trapped = true;
    // 				} else if (this.maybeTrapped) {
    // 					data.maybeTrapped = true;
    // 				}
    // 			}
    // 		} else {
    // 			this.maybeDisabled = false;
    // 			this.maybeLocked = false;
    // 			if (canSwitchIn) {
    // 				// Discovered by selecting a valid Pokémon as a switch target and cancelling.
    // 				if (this.trapped) data.trapped = true;
    // 			}
    // 			this.maybeTrapped = false;
    // 		}
    //
    // 		if (!lockedMove) {
    // 			if (this.canMegaEvo) data.canMegaEvo = true;
    // 			if (this.canMegaEvoX) data.canMegaEvoX = true;
    // 			if (this.canMegaEvoY) data.canMegaEvoY = true;
    // 			if (this.canUltraBurst) data.canUltraBurst = true;
    // 			const canZMove = this.battle.actions.canZMove(this);
    // 			if (canZMove) data.canZMove = canZMove;
    //
    // 			if (this.getDynamaxRequest()) data.canDynamax = true;
    // 			if (data.canDynamax || this.volatiles['dynamax']) data.maxMoves = this.getDynamaxRequest(true);
    // 			if (this.canTerastallize) data.canTerastallize = this.canTerastallize;
    // 		}
    //
    // 		return data;
    // 	}
    //
    pub fn get_move_request_data(&self) -> serde_json::Value {
        let moves: Vec<serde_json::Value> = self
            .move_slots
            .iter()
            .map(|slot| {
                serde_json::json!({
                    "move": slot.move_name,
                    "id": slot.id.as_str(),
                    "pp": slot.pp,
                    "maxpp": slot.maxpp,
                    "target": slot.target,
                    "disabled": slot.disabled
                })
            })
            .collect();

        serde_json::json!({
            "moves": moves,
            "canDynamax": self.can_gigantamax.is_some() || self.dynamax_level > 0
        })
    }
}
