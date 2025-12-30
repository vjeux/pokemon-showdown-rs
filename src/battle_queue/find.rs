use crate::*;
use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Find a specific action by predicate
    pub fn find<F>(&self, predicate: F) -> Option<&Action>
    where
        F: Fn(&Action) -> bool,
    {
        self.list.iter().find(|action| predicate(action))
    }
}
