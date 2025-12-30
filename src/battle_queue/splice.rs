use crate::*;
use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Remove count entries starting at index
    /// Equivalent to JavaScript array.splice(index, count)
    /// TypeScript: battle.queue.list.splice(index, count)
    pub fn splice(&mut self, index: usize, count: usize) {
        if index >= self.list.len() {
            return;
        }
        let end = std::cmp::min(index + count, self.list.len());
        self.list.drain(index..end);
    }
}
