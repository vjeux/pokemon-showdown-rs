use crate::*;
use crate::battle::FaintData;

impl Battle {

    /// Check if battle is over
    /// Check if the battle has a winner
    /// Equivalent to battle.ts checkWin()
    //
    // 	checkWin(faintData?: Battle['faintQueue'][0]) {
    // 		if (this.sides.every(side => !side.pokemonLeft)) {
    // 			this.win(faintData && this.gen > 4 ? faintData.target.side : null);
    // 			return true;
    // 		}
    // 		for (const side of this.sides) {
    // 			if (!side.foePokemonLeft()) {
    // 				this.win(side);
    // 				return true;
    // 			}
    // 		}
    // 	}
    //
    pub fn check_win(&mut self, faint_data: Option<FaintData>) -> bool {
        // JavaScript: checkWin(faintData?: Battle['faintQueue'][0])

        eprintln!("[CHECK_WIN] Called at turn={}, faint_data={:?}", self.turn, faint_data.is_some());
        eprintln!("[CHECK_WIN] Side 0 pokemon_left={}, Side 1 pokemon_left={}",
            self.sides[0].pokemon_left, self.sides[1].pokemon_left);

        // Check if all sides have no Pokemon left - tie/draw scenario
        if self.sides.iter().all(|side| side.pokemon_left == 0) {
            eprintln!("[CHECK_WIN] All sides have 0 Pokemon left - tie/draw");
            // JS: this.win(faintData && this.gen > 4 ? faintData.target.side : null);
            // In Gen 5+, the side that fainted last wins
            let winner = if let Some(faint_data) = faint_data {
                if self.gen > 4 {
                    // Extract the side_idx from faint_data.target (side_idx, poke_idx)
                    let (side_idx, _) = faint_data.target;
                    // Get the Side's ID
                    Some(self.sides[side_idx].id)
                } else {
                    None
                }
            } else {
                None
            };
            self.win(winner);
            return true;
        }

        // Check each side to see if all their foes have no Pokemon left
        // JS: for (const side of this.sides) {
        // JS:     if (!side.foePokemonLeft()) {
        // JS:         this.win(side);
        // JS:         return true;
        // JS:     }
        // JS: }
        //
        // foePokemonLeft() logic from side.ts:
        // 	foePokemonLeft() {
        // 		if (this.battle.gameType === 'freeforall') {
        // 			return this.battle.sides.filter(side => side !== this).map(side => side.pokemonLeft).reduce((a, b) => a + b);
        // 		}
        // 		if (this.foe.allySide) return this.foe.pokemonLeft + this.foe.allySide.pokemonLeft;
        // 		return this.foe.pokemonLeft;
        // 	}
        for side_idx in 0..self.sides.len() {
            // Calculate foe_pokemon_left inline since Side doesn't have access to Battle
            let foe_pokemon_left = if self.game_type == GameType::FreeForAll {
                // Sum all sides except this one
                self.sides
                    .iter()
                    .enumerate()
                    .filter(|(idx, _)| *idx != side_idx)
                    .map(|(_, s)| s.pokemon_left)
                    .sum::<usize>()
            } else if let Some(foe_idx) = self.sides[side_idx].foe_index {
                // Check if foe has ally
                if let Some(ally_idx) = self.sides.get(foe_idx).and_then(|f| f.ally_index) {
                    // Return foe.pokemonLeft + foe.allySide.pokemonLeft
                    self.sides[foe_idx].pokemon_left + self.sides[ally_idx].pokemon_left
                } else {
                    // Return foe.pokemonLeft
                    self.sides[foe_idx].pokemon_left
                }
            } else {
                // No foe set up
                continue;
            };

            eprintln!("[CHECK_WIN] Side {} foe_pokemon_left={}", side_idx, foe_pokemon_left);

            // JS: if (!side.foePokemonLeft())
            if foe_pokemon_left == 0 {
                eprintln!("[CHECK_WIN] Side {} has won! (foe has 0 Pokemon left)", side_idx);
                // JS: this.win(side);
                self.win(Some(self.sides[side_idx].id));
                return true;
            }
        }

        eprintln!("[CHECK_WIN] No winner yet");
        false
    }
}
