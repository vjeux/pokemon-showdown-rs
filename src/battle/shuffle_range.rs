use crate::*;

impl Battle {

    /// Shuffle a range of a slice in place
    /// Rust helper method - JavaScript uses prng.shuffle(list, start, end) inline
    /// This method is called from speed_sort() to shuffle tied items
    ///
    /// JavaScript (prng.js:140-148):
    ///   shuffle(items, start = 0, end = items.length) {
    ///     while (start < end - 1) {
    ///       const nextIndex = this.random(start, end);
    ///       if (start !== nextIndex) {
    ///         [items[start], items[nextIndex]] = [items[nextIndex], items[start]];
    ///       }
    ///       start++;
    ///     }
    ///   }
    pub fn shuffle_range<T>(&mut self, list: &mut [T], mut start: usize, end: usize) {
        eprintln!("[SHUFFLE_RANGE CALLED] turn={}, start={}, end={}, count={}", self.turn, start, end, end - start);

        // Log shuffle_range calls during specific turns for debugging
        let should_log = self.turn == 4 || self.turn == 5 || (self.turn >= 20 && self.turn <= 22);

        if should_log {
            eprintln!("[TURN {} SHUFFLE_RANGE] Starting shuffle of range=[{}, {}), count={} items",
                self.turn, start, end, end - start);
        }

        while start < end - 1 {
            let call_before = self.prng.call_count;
            let next_index = self.random_with_range(start as i32, end as i32) as usize;

            if should_log {
                eprintln!("[TURN {} SHUFFLE_RANGE] Call #{}: random_with_range({}, {}) = {}",
                    self.turn, call_before + 1, start, end, next_index);
            }

            if start != next_index {
                list.swap(start, next_index);
            }
            start += 1;
        }

        if should_log {
            eprintln!("[TURN {} SHUFFLE_RANGE] Finished shuffle", self.turn);
        }
    }
}
