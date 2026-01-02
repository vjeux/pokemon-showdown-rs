// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Insert action maintaining sort order
    /// This is the core insertion logic used by TypeScript's insertChoice()
    /// TypeScript insertChoice() = resolveAction() + this insert logic
    /// In Rust, we separate concerns: caller resolves action, this just inserts
    ///
    /// TypeScript source (insertChoice, lines 385-402 in battle-queue.ts):
    /// ```
    /// let firstIndex = null;
    /// let lastIndex = null;
    /// for (const [i, curAction] of this.list.entries()) {
    ///     const compared = this.battle.comparePriority(actions[0], curAction);
    ///     if (compared <= 0 && firstIndex === null) {
    ///         firstIndex = i;
    ///     }
    ///     if (compared < 0) {
    ///         lastIndex = i;
    ///         break;
    ///     }
    /// }
    /// if (firstIndex === null) {
    ///     this.list.push(...actions);
    /// } else {
    ///     if (lastIndex === null) lastIndex = this.list.length;
    ///     const index = firstIndex === lastIndex ? firstIndex : this.battle.random(firstIndex, lastIndex + 1);
    ///     this.list.splice(index, 0, ...actions);
    /// }
    /// ```
    pub fn insert_in_order(&mut self, action: Action) {
        // Find the right position based on priority
        let mut insert_pos = self.list.len();

        for (i, existing) in self.list.iter().enumerate() {
            // Order: lower first
            let order_cmp = action.order().cmp(&existing.order());
            if order_cmp == std::cmp::Ordering::Less {
                insert_pos = i;
                break;
            } else if order_cmp == std::cmp::Ordering::Greater {
                continue;
            }

            // Priority: higher first
            if action.priority() > existing.priority() {
                insert_pos = i;
                break;
            } else if action.priority() < existing.priority() {
                continue;
            }

            // Fractional priority: higher first
            if action.fractional_priority() > existing.fractional_priority() {
                insert_pos = i;
                break;
            } else if action.fractional_priority() < existing.fractional_priority() {
                continue;
            }

            // Speed: higher first
            if action.speed() > existing.speed() {
                insert_pos = i;
                break;
            }
        }

        self.list.insert(insert_pos, action);
    }
}
