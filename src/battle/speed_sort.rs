// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::battle::PriorityItem;

impl Battle {

    /// Sort a list, resolving speed ties randomly (the way the games do)
    /// Equivalent to battle.ts speedSort()
    // TypeScript source:
    // /** Sort a list, resolving speed ties the way the games do. */
    // 	speedSort<T extends AnyObject>(list: T[], comparator: (a: T, b: T) => number = this.comparePriority) {
    // 		if (list.length < 2) return;
    // 		let sorted = 0;
    // 		// This is a Selection Sort - not the fastest sort in general, but
    // 		// actually faster than QuickSort for small arrays like the ones
    // 		// `speedSort` is used for.
    // 		// More importantly, it makes it easiest to resolve speed ties
    // 		// properly.
    // 		while (sorted + 1 < list.length) {
    // 			let nextIndexes = [sorted];
    // 			// grab list of next indexes
    // 			for (let i = sorted + 1; i < list.length; i++) {
    // 				const delta = comparator(list[nextIndexes[0]], list[i]);
    // 				if (delta < 0) continue;
    // 				if (delta > 0) nextIndexes = [i];
    // 				if (delta === 0) nextIndexes.push(i);
    // 			}
    // 			// put list of next indexes where they belong
    // 			for (let i = 0; i < nextIndexes.length; i++) {
    // 				const index = nextIndexes[i];
    // 				if (index !== sorted + i) {
    // 					// nextIndexes is guaranteed to be in order, so it will never have
    // 					// been disturbed by an earlier swap
    // 					[list[sorted + i], list[index]] = [list[index], list[sorted + i]];
    // 				}
    // 			}
    // 			if (nextIndexes.length > 1) {
    // 				this.prng.shuffle(list, sorted, sorted + nextIndexes.length);
    // 			}
    // 			sorted += nextIndexes.length;
    // 		}
    // 	}
    //
    pub fn speed_sort<T, F>(&mut self, list: &mut [T], mut get_priority: F)
    where
        F: FnMut(&T) -> PriorityItem,
    {
        if list.len() < 2 {
            return;
        }

        // Selection sort with random tie-breaking
        let mut sorted = 0;
        while sorted + 1 < list.len() {
            let mut next_indexes = vec![sorted];

            // Find the next item(s) with highest priority
            for i in (sorted + 1)..list.len() {
                let priority_a = get_priority(&list[next_indexes[0]]);
                let priority_i = get_priority(&list[i]);
                let cmp = Self::compare_priority(&priority_a, &priority_i);

                match cmp {
                    std::cmp::Ordering::Less => continue,
                    std::cmp::Ordering::Greater => next_indexes = vec![i],
                    std::cmp::Ordering::Equal => {
                        next_indexes.push(i);
                    }
                }
            }

            // Put the next items where they belong
            for (offset, &index) in next_indexes.iter().enumerate() {
                if index != sorted + offset {
                    list.swap(sorted + offset, index);
                }
            }

            // If there were ties, shuffle them randomly
            if next_indexes.len() > 1 {
                let end = sorted + next_indexes.len();
                eprintln!("[SPEED_SORT] Found {} tied items at sorted={}, shuffling range [{}, {})",
                    next_indexes.len(), sorted, sorted, end);
                self.shuffle_range(list, sorted, end);
                eprintln!("[SPEED_SORT] Shuffle complete");
            }

            sorted += next_indexes.len();
        }
    }
}
