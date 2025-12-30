use crate::side::*;
use crate::*;

impl Side {

    /// Choose team action (team preview)
    //
    // 	chooseTeam(data?: string) {
    // 		if (this.requestState !== 'teampreview') {
    // 			return this.emitChoiceError(`Can't choose for Team Preview: You're not in a Team Preview phase`);
    // 		}
    //
    // 		const ruleTable = this.battle.ruleTable;
    // 		let positions = data ? data.split(data.includes(',') ? ',' : '').map(datum => parseInt(datum) - 1) :
    // 			[...this.pokemon.keys()]; // autoChoose
    // 		const pickedTeamSize = this.pickedTeamSize();
    //
    // 		// make sure positions is exactly of length pickedTeamSize
    // 		// - If too big: the client automatically sends a full list, so we just trim it down to size
    // 		positions.splice(pickedTeamSize);
    // 		// - If too small: we intentionally support only sending leads and having the sim fill in the rest
    // 		if (positions.length < pickedTeamSize) {
    // 			for (let i = 0; i < pickedTeamSize; i++) {
    // 				if (!positions.includes(i)) positions.push(i);
    // 				// duplicate in input, let the rest of the code handle the error message
    // 				if (positions.length >= pickedTeamSize) break;
    // 			}
    // 		}
    //
    // 		for (const [index, pos] of positions.entries()) {
    // 			if (isNaN(pos) || pos < 0 || pos >= this.pokemon.length) {
    // 				return this.emitChoiceError(`Can't choose for Team Preview: You do not have a Pokémon in slot ${pos + 1}`);
    // 			}
    // 			if (positions.indexOf(pos) !== index) {
    // 				return this.emitChoiceError(`Can't choose for Team Preview: The Pokémon in slot ${pos + 1} can only switch in once`);
    // 			}
    // 		}
    //
    // 		const result = ruleTable.onChooseTeam?.[0].call(this.battle, positions, this.pokemon, !data);
    // 		if (result) {
    // 			if (typeof result === 'string') {
    // 				return this.emitChoiceError(`Can't choose for Team Preview: ${result}`);
    // 			}
    // 			if (result.length < pickedTeamSize) {
    // 				throw new Error(`onChooseTeam from ${ruleTable.onChooseTeam![1]} returned a team of size ${result.length}, which is less than the required size of ${pickedTeamSize}`);
    // 			}
    // 			positions = result.slice(0, pickedTeamSize);
    // 		}
    //
    // 		for (const [index, pos] of positions.entries()) {
    // 			this.choice.switchIns.add(pos);
    // 			this.choice.actions.push({
    // 				choice: 'team',
    // 				index,
    // 				pokemon: this.pokemon[pos],
    // 				priority: -index,
    // 			} as ChosenAction);
    // 		}
    //
    // 		return true;
    // 	}
    //
    pub fn choose_team(&mut self, positions: Vec<usize>) -> Result<(), String> {
        if self.request_state != RequestState::TeamPreview {
            return Err("Not in team preview phase".to_string());
        }

        for (i, pos) in positions.iter().enumerate() {
            if *pos >= self.pokemon.len() {
                return Err(format!("No Pokemon in slot {}", pos + 1));
            }
            if positions[..i].contains(pos) {
                return Err(format!("Pokemon in slot {} selected twice", pos + 1));
            }
        }

        for (index, pos) in positions.iter().enumerate() {
            self.choice.switch_ins.push(*pos);
            self.choice.actions.push(ChosenAction {
                choice: ChoiceType::Team,
                pokemon_index: *pos,
                target_loc: None,
                move_id: None,
                switch_index: Some(index),
                mega: false,
                zmove: None,
                max_move: None,
                terastallize: None,
            });
        }

        Ok(())
    }
}
