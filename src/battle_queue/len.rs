use crate::*;
use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Get the number of actions
    pub fn len(&self) -> usize {
        self.list.len()
    }
}
