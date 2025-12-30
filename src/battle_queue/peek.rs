use crate::*;
use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Peek at the next action without removing
    // 	peek(end?: boolean): Action | undefined {
    // 		return this.list[end ? this.list.length - 1 : 0];
    // 	}
    //
    pub fn peek(&self) -> Option<&Action> {
        self.list.first()
    }
}
