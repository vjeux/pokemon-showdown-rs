use crate::battle_queue::BattleQueue;
use crate::Battle;

impl BattleQueue {

    /// Sort the queue by priority
    /// Order: order (lower first), priority (higher first), speed (higher first)
    ///
    /// JavaScript speedSort implementation:
    /// ```javascript
    /// speedSort<T extends AnyObject>(list: T[], comparator: (a: T, b: T) => number = this.comparePriority) {
    ///     if (list.length < 2) return;
    ///     let sorted = 0;
    ///     // This is a Selection Sort - not the fastest sort in general, but
    ///     // actually faster than QuickSort for small arrays like the ones
    ///     // `speedSort` is used for.
    ///     // More importantly, it makes it easiest to resolve speed ties
    ///     // properly.
    ///     while (sorted + 1 < list.length) {
    ///         let nextIndexes = [sorted];
    ///         // grab list of next indexes
    ///         for (let i = sorted + 1; i < list.length; i++) {
    ///             const delta = comparator(list[nextIndexes[0]], list[i]);
    ///             if (delta < 0) continue;
    ///             if (delta > 0) nextIndexes = [i];
    ///             if (delta === 0) nextIndexes.push(i);
    ///         }
    ///         // put list of next indexes where they belong
    ///         for (let i = 0; i < nextIndexes.length; i++) {
    ///             const index = nextIndexes[i];
    ///             if (index !== sorted + i) {
    ///                 // nextIndexes is guaranteed to be in order, so it will never have
    ///                 // been disturbed by an earlier swap
    ///                 [list[sorted + i], list[index]] = [list[index], list[sorted + i]];
    ///             }
    ///         }
    ///         if (nextIndexes.length > 1) {
    ///             this.prng.shuffle(list, sorted, sorted + nextIndexes.length);
    ///         }
    ///         sorted += nextIndexes.length;
    ///     }
    /// }
    /// ```
    ///
    /// **CRITICAL DIFFERENCE from standard sort:**
    /// When multiple actions have the same priority/speed (ties), JavaScript
    /// **shuffles them randomly using PRNG**. This is essential for correct
    /// battle simulation! A standard stable sort would keep them in insertion
    /// order, causing divergence from JavaScript.
    ///
    pub fn sort(&mut self, battle: &mut Battle) {
        // if (list.length < 2) return;
        if self.list.len() < 2 {
            return;
        }

        // let sorted = 0;
        let mut sorted = 0;

        // while (sorted + 1 < list.length)
        while sorted + 1 < self.list.len() {
            // let nextIndexes = [sorted];
            let mut next_indexes = vec![sorted];

            // grab list of next indexes
            // for (let i = sorted + 1; i < list.length; i++)
            for i in (sorted + 1)..self.list.len() {
                // const delta = comparator(list[nextIndexes[0]], list[i]);
                let delta = self.compare_actions(&self.list[next_indexes[0]], &self.list[i]);

                // if (delta < 0) continue;
                if delta < 0 {
                    continue;
                }

                // if (delta > 0) nextIndexes = [i];
                if delta > 0 {
                    next_indexes = vec![i];
                }

                // if (delta === 0) nextIndexes.push(i);
                if delta == 0 {
                    next_indexes.push(i);
                }
            }

            // put list of next indexes where they belong
            // for (let i = 0; i < nextIndexes.length; i++)
            for i in 0..next_indexes.len() {
                let index = next_indexes[i];
                // if (index !== sorted + i)
                if index != sorted + i {
                    // [list[sorted + i], list[index]] = [list[index], list[sorted + i]];
                    self.list.swap(sorted + i, index);
                }
            }

            // if (nextIndexes.length > 1) {
            //     this.prng.shuffle(list, sorted, sorted + nextIndexes.length);
            // }
            if next_indexes.len() > 1 {
                // CRITICAL: Shuffle tied elements using PRNG!
                // This randomizes the order of actions with the same priority/speed,
                // which is essential for matching JavaScript behavior.
                battle.shuffle_range(&mut self.list, sorted, sorted + next_indexes.len());
            }

            // sorted += nextIndexes.length;
            sorted += next_indexes.len();
        }
    }

    /// Compare two actions for sorting
    /// Returns: negative if a < b, positive if a > b, zero if equal
    ///
    /// Order: order (lower first), priority (higher first), fractional priority (higher first), speed (higher first)
    fn compare_actions(&self, a: &crate::battle_queue::Action, b: &crate::battle_queue::Action) -> i32 {
        // Order: lower first (so a.order - b.order)
        let order_diff = a.order() as i32 - b.order() as i32;
        if order_diff != 0 {
            return order_diff;
        }

        // Priority: higher first (so b.priority - a.priority)
        let priority_diff = b.priority() as i32 - a.priority() as i32;
        if priority_diff != 0 {
            return priority_diff;
        }

        // Fractional priority: higher first
        let frac_a = a.fractional_priority();
        let frac_b = b.fractional_priority();
        if frac_a != frac_b {
            // Higher fractional priority wins (comes first)
            if frac_b > frac_a {
                return 1; // b comes first
            } else {
                return -1; // a comes first
            }
        }

        // Speed: higher first
        let speed_a = a.speed();
        let speed_b = b.speed();
        if speed_a != speed_b {
            // Higher speed wins (comes first)
            if speed_b > speed_a {
                return 1; // b comes first
            } else {
                return -1; // a comes first
            }
        }

        // Equal
        0
    }
}
