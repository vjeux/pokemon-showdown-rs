use crate::*;
use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Get the next action (removes from front)
    // TypeScript source:
    //
    //
    // 	shift() {
    // 		return this.list.shift();
    // 	}
    //
    pub fn shift(&mut self) -> Option<Action> {
        if self.list.is_empty() {
            None
        } else {
            Some(self.list.remove(0))
        }
    }
}
