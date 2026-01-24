use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Unshift an action to the front
    // 	unshift(action: Action) {
    // 		return this.list.unshift(action);
    // 	}
    //
    pub fn unshift(&mut self, action: Action) {
        self.list.push_front(action);
    }
}
