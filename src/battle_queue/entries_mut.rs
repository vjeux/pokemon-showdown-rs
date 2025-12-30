use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Get mutable entries
    pub fn entries_mut(&mut self) -> impl Iterator<Item = (usize, &mut Action)> {
        self.list.iter_mut().enumerate()
    }
}
