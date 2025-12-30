use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Get a mutable iterator over the actions
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Action> {
        self.list.iter_mut()
    }
}
