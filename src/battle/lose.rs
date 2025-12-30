use crate::*;
use crate::choice::Choice;
use crate::side::RequestState;

impl Battle {

    /// Force a side to lose
    /// Equivalent to battle.ts lose() (battle.ts:1499-1518)
    //
    // 	lose(side: SideID | Side) {
    // 		if (typeof side === 'string') {
    // 			side = this.getSide(side);
    // 		}
    // 		if (!side) return; // can happen if a battle crashes
    // 		if (this.gameType !== 'freeforall') {
    // 			return this.win(side.foe);
    // 		}
    // 		if (!side.pokemonLeft) return;
    //
    // 		side.pokemonLeft = 0;
    // 		side.active[0]?.faint();
    // 		this.faintMessages(false, true);
    // 		if (!this.ended && side.requestState) {
    // 			side.emitRequest({ wait: true, side: side.getRequestData() });
    // 			side.clearChoice();
    // 			if (this.allChoicesDone()) this.commitChoices();
    // 		}
    // 		return true;
    // 	}
    //
    pub fn lose(&mut self, side_id: SideID) {
        // JavaScript: if (typeof side === 'string') side = this.getSide(side);
        // (Rust already has SideID, no conversion needed)

        // JavaScript: if (!side) return; // can happen if a battle crashes
        // (Rust SideID is always valid)

        // JavaScript: if (this.gameType !== 'freeforall') return this.win(side.foe);
        if self.game_type != GameType::FreeForAll {
            let foe_id = match side_id {
                SideID::P1 => SideID::P2,
                SideID::P2 => SideID::P1,
                SideID::P3 => SideID::P4,
                SideID::P4 => SideID::P3,
            };
            self.win(Some(foe_id));
            return;
        }

        // JavaScript: if (!side.pokemonLeft) return;
        if let Some(side) = self.sides.get(side_id.index()) {
            if side.pokemon_left == 0 {
                return;
            }
        }

        // JavaScript: side.pokemonLeft = 0;
        if let Some(side) = self.sides.get_mut(side_id.index()) {
            side.pokemon_left = 0;

            // JavaScript: side.active[0]?.faint();
            if let Some(Some(poke_idx)) = side.active.first() {
                if let Some(pokemon) = side.pokemon.get_mut(*poke_idx) {
                    pokemon.faint();
                }
            }
        }

        // JavaScript: this.faintMessages(false, true);
        self.faint_messages(false, true, true);

        // JavaScript: if (!this.ended && side.requestState) { ... }
        if !self.ended {
            if let Some(side) = self.sides.get_mut(side_id.index()) {
                if side.request_state != RequestState::None {
                    // JavaScript: side.emitRequest({ wait: true, side: side.getRequestData() });
                    // (We don't have emitRequest in Rust, skip for now)

                    // JavaScript: side.clearChoice();
                    side.choice = crate::side::Choice::new();

                    // JavaScript: if (this.allChoicesDone()) this.commitChoices();
                    // (Would need to implement allChoicesDone and commitChoices)
                }
            }
        }
    }
}
