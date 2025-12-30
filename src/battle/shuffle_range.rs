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
        let bt = std::backtrace::Backtrace::force_capture();
        let bt_str = format!("{}", bt);
        let caller = bt_str.lines()
            .filter(|line| line.contains("speed_sort") || line.contains("each_event") || line.contains("commit") || line.contains("run_"))
            .take(3)
            .collect::<Vec<_>>()
            .join(" <- ");
        eprintln!("[SHUFFLE_RANGE] range=[{}, {}), caller: {}", start, end, caller);

        while start < end - 1 {
            let next_index = self.random_with_range(start as i32, end as i32) as usize;
            if start != next_index {
                list.swap(start, next_index);
            }
            start += 1;
        }
    }
}
