use crate::side::*;

impl Side {

    /// Choose shift action (triples)
    //
    // 	chooseShift() {
    // 		const index = this.getChoiceIndex();
    // 		if (index >= this.active.length) {
    // 			return this.emitChoiceError(`Can't shift: You do not have a Pokémon in slot ${index + 1}`);
    // 		} else if (this.requestState !== 'move') {
    // 			return this.emitChoiceError(`Can't shift: You can only shift during a move phase`);
    // 		} else if (this.battle.gameType !== 'triples') {
    // 			return this.emitChoiceError(`Can't shift: You can only shift to the center in triples`);
    // 		} else if (index === 1) {
    // 			return this.emitChoiceError(`Can't shift: You can only shift from the edge to the center`);
    // 		}
    // 		const pokemon: Pokemon = this.active[index];
    //
    // 		this.choice.actions.push({
    // 			choice: 'shift',
    // 			pokemon,
    // 		} as ChosenAction);
    //
    // 		return true;
    // 	}
    //
    pub fn choose_shift(&mut self) -> Result<(), String> {
        let index = self.get_choice_index(false);
        if index >= self.active.len() {
            return Err(format!("No Pokemon in slot {}", index + 1));
        }
        if self.request_state != RequestState::Move {
            return Err("Can only shift during move phase".to_string());
        }
        if self.active.len() != 3 {
            return Err("Can only shift in triples".to_string());
        }
        if index == 1 {
            return Err("Can only shift from edge to center".to_string());
        }

        self.choice.actions.push(ChosenAction {
            choice: ChoiceType::Shift,
            pokemon_index: index,
            target_loc: None,
            move_id: None,
            switch_index: None,
            mega: false,
            zmove: None,
            max_move: None,
            terastallize: None,
        });

        Ok(())
    }
}
