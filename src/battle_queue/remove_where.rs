use crate::*;
use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Remove actions matching predicate
    pub fn remove_where<F>(&mut self, predicate: F) -> Vec<Action>
    where
        F: Fn(&Action) -> bool,
    {
        let mut removed = Vec::new();
        let mut i = 0;
        while i < self.list.len() {
            if predicate(&self.list[i]) {
                removed.push(self.list.remove(i));
            } else {
                i += 1;
            }
        }
        removed
    }
}
