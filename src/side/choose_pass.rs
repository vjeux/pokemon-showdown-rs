use crate::side::*;
use crate::*;

impl Side {

    /// Choose pass action
    //
    // 	choosePass(): boolean | Side {
    // 		const index = this.getChoiceIndex(true);
    // 		if (index >= this.active.length) return false;
    // 		const pokemon: Pokemon = this.active[index];
    //
    // 		switch (this.requestState) {
    // 		case 'switch':
    // 			if (pokemon.switchFlag) { // This condition will always happen if called by Battle#choose()
    // 				if (!this.choice.forcedPassesLeft) {
    // 					return this.emitChoiceError(`Can't pass: You need to switch in a Pokémon to replace ${pokemon.name}`);
    // 				}
    // 				this.choice.forcedPassesLeft--;
    // 			}
    // 			break;
    // 		case 'move':
    // 			if (!pokemon.fainted && !pokemon.volatiles['commanding']) {
    // 				return this.emitChoiceError(`Can't pass: Your ${pokemon.name} must make a move (or switch)`);
    // 			}
    // 			break;
    // 		default:
    // 			return this.emitChoiceError(`Can't pass: Not a move or switch request`);
    // 		}
    //
    // 		this.choice.actions.push({
    // 			choice: 'pass',
    // 		} as ChosenAction);
    // 		return true;
    // 	}
    //
    pub fn choose_pass(&mut self) -> bool {
        let index = self.get_choice_index(true);
        if index >= self.active.len() {
            return false;
        }

        match self.request_state {
            RequestState::Switch => {
                if self.choice.forced_passes_left == 0 {
                    return false;
                }
                self.choice.forced_passes_left -= 1;
            }
            RequestState::Move => {
                // Check if the Pokemon is fainted
                if let Some(Some(pokemon_idx)) = self.active.get(index) {
                    if let Some(pokemon) = self.pokemon.get(*pokemon_idx) {
                        if !pokemon.is_fainted() {
                            return false; // Can't pass if not fainted
                        }
                    }
                }
            }
            _ => return false,
        }

        self.choice.actions.push(ChosenAction {
            choice: ChoiceType::Pass,
            pokemon_index: index,
            target_loc: None,
            move_id: None,
            switch_index: None,
            mega: false,
            zmove: None,
            max_move: None,
            terastallize: None,
        });
        true
    }
}
