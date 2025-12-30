use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Cancel all actions for a specific pokemon
    //
    // 	cancelAction(pokemon: Pokemon) {
    // 		const oldLength = this.list.length;
    // 		for (let i = 0; i < this.list.length; i++) {
    // 			if (this.list[i].pokemon === pokemon) {
    // 				this.list.splice(i, 1);
    // 				i--;
    // 			}
    // 		}
    // 		return this.list.length !== oldLength;
    // 	}
    //
    pub fn cancel_action(&mut self, side_index: usize, pokemon_index: usize) -> bool {
        let old_len = self.list.len();
        self.list.retain(|action| {
            !(action.side_index() == Some(side_index)
                && action.pokemon_index() == Some(pokemon_index))
        });
        self.list.len() != old_len
    }
}
