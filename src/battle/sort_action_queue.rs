// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::battle::PriorityItem;

impl Battle {
    /// Sort the action queue using the default comparator
    /// Equivalent to JavaScript's this.queue.sort()
    //
    // TypeScript:
    // sort() {
    //     this.battle.speedSort(this.list);
    //     return this;
    // }
    //
    // Which calls speedSort with the default comparator (comparePriority)
    pub fn sort_action_queue(&mut self) {
        let mut list = std::mem::take(&mut self.queue.list);

        // JavaScript: this.battle.speedSort(this.list);
        // speedSort uses comparePriority by default, which compares:
        // 1. order (low to high, default last = 4294967296)
        // 2. priority (high to low, default 0)
        // 3. speed (high to low, default 0)
        // 4. subOrder (low to high, default 0)
        // 5. effectOrder (low to high, default 0)
        self.speed_sort(&mut list, |action| {
            PriorityItem {
                order: Some(action.order()),
                priority: action.priority() as i32,
                speed: action.speed(),
                sub_order: 0,
                effect_order: 0,
                index: 0,
            }
        });

        self.queue.list = list;
    }
}
