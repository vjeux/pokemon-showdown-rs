// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {
    /// Sort the action queue using the default comparator
    /// Equivalent to JavaScript's this.queue.sort()
    ///
    /// JavaScript equivalent:
    /// ```javascript
    /// sort() {
    ///     this.battle.speedSort(this.list);
    ///     return this;
    /// }
    /// ```
    ///
    /// This delegates to BattleQueue::sort() which implements JavaScript's
    /// speedSort algorithm (selection sort with PRNG shuffling for ties).
    pub fn sort_action_queue(&mut self) {
        // Take ownership of the queue temporarily to satisfy borrow checker
        let mut queue = std::mem::take(&mut self.queue);

        // JavaScript: this.battle.speedSort(this.list);
        // The new BattleQueue::sort() implementation matches JavaScript's
        // speedSort algorithm, including randomizing tied elements
        queue.sort(self);

        // Restore the queue
        self.queue = queue;
    }
}
