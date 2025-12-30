use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;
use crate::battle_queue::SwitchAction;

impl BattleQueue {

    /// Check if a pokemon will switch this turn
    //
    // 	willSwitch(pokemon: Pokemon) {
    // 		for (const action of this.list) {
    // 			if (['switch', 'instaswitch'].includes(action.choice) && action.pokemon === pokemon) {
    // 				return action;
    // 			}
    // 		}
    // 		return null;
    // 	}
    //
    pub fn will_switch(&self, side_index: usize, pokemon_index: usize) -> Option<&SwitchAction> {
        for action in &self.list {
            if let Action::Switch(switch_action) = action {
                if switch_action.side_index == side_index
                    && switch_action.pokemon_index == pokemon_index
                {
                    return Some(switch_action);
                }
            }
        }
        None
    }
}
