use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;
use crate::battle_queue::MoveAction;

impl BattleQueue {

    /// Check if a pokemon will move this turn
    //
    // 	willMove(pokemon: Pokemon) {
    // 		if (pokemon.fainted) return null;
    // 		for (const action of this.list) {
    // 			if (action.choice === 'move' && action.pokemon === pokemon) {
    // 				return action;
    // 			}
    // 		}
    // 		return null;
    // 	}
    //
    pub fn will_move(&self, side_index: usize, pokemon_index: usize) -> Option<&MoveAction> {
        for action in &self.list {
            if let Action::Move(move_action) = action {
                if move_action.side_index == side_index
                    && move_action.pokemon_index == pokemon_index
                {
                    return Some(move_action);
                }
            }
        }
        None
    }
}
