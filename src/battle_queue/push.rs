use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Push an action to the end
    // 	push(action: Action) {
    // 		return this.list.push(action);
    // 	}
    //
    pub fn push(&mut self, action: Action) {
        self.list.push_back(action);
    }
}
