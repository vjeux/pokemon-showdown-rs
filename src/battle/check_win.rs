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

        // Check if all sides have no Pokemon left - tie/draw scenario
        if self.sides.iter().all(|side| side.pokemon_left == 0) {
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
        for side in self.sides.iter() {
            // JS: if (!side.foePokemonLeft())
            if side.foe_pokemon_left() == 0 {
                // JS: this.win(side);
                self.win(Some(side.id));
                return true;
            }
        }

        false
    }
}
