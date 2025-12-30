use crate::*;
use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Cancel move action for a specific pokemon
    //
    // 	cancelMove(pokemon: Pokemon) {
    // 		for (const [i, action] of this.list.entries()) {
    // 			if (action.choice === 'move' && action.pokemon === pokemon) {
    // 				this.list.splice(i, 1);
    // 				return true;
    // 			}
    // 		}
    // 		return false;
    // 	}
    //
    pub fn cancel_move(&mut self, side_index: usize, pokemon_index: usize) -> bool {
        let pos = self.list.iter().position(|action| {
            action.is_move()
                && action.side_index() == Some(side_index)
                && action.pokemon_index() == Some(pokemon_index)
        });
        if let Some(i) = pos {
            self.list.remove(i);
            true
        } else {
            false
        }
    }
}
