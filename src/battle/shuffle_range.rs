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
        eprintln!("DEBUG: shuffle_range called with start={}, end={}, length={}", start, end, end - start);
        while start < end - 1 {
            let next_index = self.random_range(start as i32, end as i32) as usize;
            eprintln!("DEBUG: shuffle iteration: start={}, next_index={}", start, next_index);
            if start != next_index {
                list.swap(start, next_index);
            }
            start += 1;
        }
        eprintln!("DEBUG: shuffle_range complete");
    }
}
