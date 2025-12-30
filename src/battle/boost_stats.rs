use crate::*;
use std::collections::HashMap;

impl Battle {

    /// Helper to boost stats from a HashMap
    /// Rust helper method - JavaScript boost() accepts SparseBoostsTable (object with stat names as keys)
    #[allow(dead_code)]
    /// This helper converts HashMap format to the Vec<(&str, i8)> format used by boost()
    /// Allows calling boost() when stat boosts are stored in HashMap format
    pub fn boost_stats(
        &mut self,
        target_side: usize,
        target_idx: usize,
        boosts_map: &HashMap<String, i32>,
    ) {
        // Convert HashMap to Vec of tuples for boost method
        let mut boosts: Vec<(&str, i8)> = Vec::new();
        for (stat, &val) in boosts_map.iter() {
            boosts.push((stat.as_str(), val as i8));
        }

        self.boost(&boosts, (target_side, target_idx), None, None);
    }
}
