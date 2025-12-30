use crate::*;
use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Change an action for a pokemon (cancel and reinsert)
    // TypeScript source:
    // /**
    // 	 * Changes a pokemon's action, and inserts its new action
    // 	 * in priority order.
    // 	 *
    // 	 * You'd normally want the OverrideAction event (which doesn't
    // 	 * change priority order).
    // 	 */
    // 	changeAction(pokemon: Pokemon, action: ActionChoice) {
    // 		this.cancelAction(pokemon);
    // 		if (!action.pokemon) action.pokemon = pokemon;
    // 		this.insertChoice(action);
    // 	}
    //
    pub fn change_action(&mut self, side_index: usize, pokemon_index: usize, new_action: Action) {
        self.cancel_action(side_index, pokemon_index);
        // Insert in priority order
        self.insert_in_order(new_action);
    }
}
