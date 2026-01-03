use crate::*;
use crate::side::RequestState;
use crate::Pokemon;

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
        // JavaScript: side.active[0]?.faint();
        let pokemon_to_faint = {
            if let Some(side) = self.sides.get_mut(side_id.index()) {
                side.pokemon_left = 0;

                // Get the first active Pokemon position
                side.active.first().and_then(|opt_idx| {
                    opt_idx.map(|poke_idx| (side_id.index(), poke_idx))
                })
            } else {
                None
            }
        };

        // Faint the Pokemon if there was one active
        if let Some(pokemon_pos) = pokemon_to_faint {
            Pokemon::faint(self, pokemon_pos, None, None);
        }

        // JavaScript: this.faintMessages(false, true);
        self.faint_messages(false, true, true);

        // JavaScript: if (!this.ended && side.requestState) { ... }
        if !self.ended {
            let side_idx = side_id.index();
            if self.sides[side_idx].request_state != RequestState::None {
                // JavaScript: side.emitRequest({ wait: true, side: side.getRequestData() });
                let request = serde_json::json!({
                    "wait": true,
                    "side": self.sides[side_idx].get_request_data(),
                });
                self.sides[side_idx].emit_request(&request);

                // JavaScript: side.clearChoice();
                self.sides[side_idx].choice = crate::side::Choice::new();
            }

            // JavaScript: if (this.allChoicesDone()) this.commitChoices();
            if self.all_choices_done() {
                self.commit_choices();
            }
        }
    }
}
