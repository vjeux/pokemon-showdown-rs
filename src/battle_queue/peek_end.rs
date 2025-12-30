use crate::*;
use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Peek at the last action
    pub fn peek_end(&self) -> Option<&Action> {
        self.list.last()
    }
}
