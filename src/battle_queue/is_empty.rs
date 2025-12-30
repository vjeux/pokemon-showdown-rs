use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}
