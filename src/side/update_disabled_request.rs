use crate::side::*;
use crate::*;

impl Side {

    /// Update disabled moves in request
    /// Equivalent to side.ts updateDisabledRequest()
    //
    // 	updateDisabledRequest(pokemon: Pokemon, req: PokemonMoveRequestData) {
    // 		let updated = false;
    // 		if (pokemon.maybeLocked) {
    // 			pokemon.maybeLocked = false;
    // 			delete req.maybeLocked;
    // 			updated = true;
    // 		}
    // 		if (pokemon.maybeDisabled && this.battle.gameType !== 'singles') {
    // 			if (this.battle.gen >= 4) {
    // 				pokemon.maybeDisabled = false;
    // 				delete req.maybeDisabled;
    // 				updated = true;
    // 			}
    // 			for (const m of req.moves) {
    // 				const disabled = pokemon.getMoveData(m.id)?.disabled;
    // 				if (disabled && (this.battle.gen >= 4 || this.battle.actions.targetTypeChoices(m.target!))) {
    // 					m.disabled = true;
    // 					updated = true;
    // 				}
    // 			}
    // 		}
    // 		if (req.moves.every(m => m.disabled || m.id === 'struggle')) {
    // 			if (req.canMegaEvo) {
    // 				req.canMegaEvo = false;
    // 				updated = true;
    // 			}
    // 			if (req.canMegaEvoX) {
    // 				req.canMegaEvoX = false;
    // 				updated = true;
    // 			}
    // 			if (req.canMegaEvoY) {
    // 				req.canMegaEvoY = false;
    // 				updated = true;
    // 			}
    // 			if (req.canUltraBurst) {
    // 				req.canUltraBurst = false;
    // 				updated = true;
    // 			}
    // 			if (req.canZMove) {
    // 				req.canZMove = undefined;
    // 				updated = true;
    // 			}
    // 			if (req.canDynamax) {
    // 				req.canDynamax = false;
    // 				delete req.maxMoves;
    // 				updated = true;
    // 			}
    // 			if (req.canTerastallize) {
    // 				req.canTerastallize = undefined;
    // 				updated = true;
    // 			}
    // 		}
    // 		return updated;
    // 	}
    //
    pub fn update_disabled_request(&mut self, pokemon_idx: usize) {
        // In the full implementation, this would update move disabled states
        // For now, just iterate through moves and check PP
        if let Some(pokemon) = self.pokemon.get_mut(pokemon_idx) {
            for slot in &mut pokemon.move_slots {
                if slot.pp == 0 {
                    slot.disabled = true;
                }
            }
        }
    }
}
