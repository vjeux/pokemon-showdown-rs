use crate::*;
use crate::battle_queue::Action;

impl Battle {
    /// Helper to call queue.insertChoice() from Battle methods
    /// JavaScript equivalent: this.queue.insertChoice({ choice: ... })
    ///
    /// Due to Rust's borrow checker, we can't call self.queue.insert_choice(self, ...)
    /// directly, so this helper method does the call by splitting the borrows.
    pub fn queue_insert_choice(&mut self, action: Action) {
        // Split mutable borrows: we need both &mut queue and &mut battle (for PRNG)
        // This is safe because queue.insert_choice only uses Battle for PRNG,
        // which doesn't touch the queue
        let queue_ptr = &mut self.queue as *mut crate::battle_queue::BattleQueue;
        let battle_ptr = self as *mut Battle;

        unsafe {
            // SAFETY: We're splitting the mutable borrow of self into:
            // 1. &mut self.queue (via queue_ptr)
            // 2. &mut self (via battle_ptr) - but insert_choice only uses PRNG
            // These don't alias because insert_choice doesn't modify queue through battle
            (*queue_ptr).insert_choice(&mut *battle_ptr, action);
        }
    }
}
