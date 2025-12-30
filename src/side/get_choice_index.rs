use crate::side::*;
use crate::*;

impl Side {

    /// Get the current choice action index
    //
    // 	getChoiceIndex(isPass?: boolean) {
    // 		let index = this.choice.actions.length;
    //
    // 		if (!isPass) {
    // 			switch (this.requestState) {
    // 			case 'move':
    // 				// auto-pass
    // 				while (
    // 					index < this.active.length &&
    // 					(this.active[index].fainted || this.active[index].volatiles['commanding'])
    // 				) {
    // 					this.choosePass();
    // 					index++;
    // 				}
    // 				break;
    // 			case 'switch':
    // 				while (index < this.active.length && !this.active[index].switchFlag) {
    // 					this.choosePass();
    // 					index++;
    // 				}
    // 				break;
    // 			}
    // 		}
    //
    // 		return index;
    // 	}
    //
    pub fn get_choice_index(&mut self, is_pass: bool) -> usize {
        let mut index = self.choice.actions.len();

        if !is_pass {
            match self.request_state {
                RequestState::Move => {
                    // auto-pass for fainted Pokemon or those with 'commanding' volatile
                    while index < self.active.len() {
                        if let Some(Some(pokemon_idx)) = self.active.get(index) {
                            if let Some(pokemon) = self.pokemon.get(*pokemon_idx) {
                                use crate::dex_data::ID;
                                let is_fainted = pokemon.is_fainted();
                                let has_commanding = pokemon.volatiles.contains_key(&ID::from("commanding"));

                                if is_fainted || has_commanding {
                                    self.choose_pass();
                                    index += 1;
                                    continue;
                                }
                            }
                        }
                        break;
                    }
                }
                RequestState::Switch => {
                    // auto-pass for Pokemon without switchFlag
                    while index < self.active.len() {
                        if let Some(Some(pokemon_idx)) = self.active.get(index) {
                            if let Some(pokemon) = self.pokemon.get(*pokemon_idx) {
                                if !pokemon.switch_flag {
                                    self.choose_pass();
                                    index += 1;
                                    continue;
                                }
                            }
                        }
                        break;
                    }
                }
                _ => {}
            }
        }

        index
    }
}
