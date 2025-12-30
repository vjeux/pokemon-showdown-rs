use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Check if any pokemon will act
    //
    // 	willAct() {
    // 		for (const action of this.list) {
    // 			if (['move', 'switch', 'instaswitch', 'shift'].includes(action.choice)) {
    // 				return action;
    // 			}
    // 		}
    // 		return null;
    // 	}
    //
    pub fn will_act(&self) -> bool {
        self.list
            .iter()
            .any(|action| matches!(action, Action::Move(_) | Action::Switch(_)))
    }
}
