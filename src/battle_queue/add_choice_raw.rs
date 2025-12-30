use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Add a pre-resolved action directly to the queue
    /// Equivalent to `this.queue.list.push(action)` in TypeScript
    /// This is a borrow-checker helper - use when you already have a resolved Action
    /// and don't need the resolution logic from addChoice()
    pub fn add_choice_raw(&mut self, action: Action) {
        self.list.push(action);
    }
}
