use crate::*;

impl<'a> BattleActions<'a> {

    /// Get multi-hit count for a move
    /// Equivalent to multi-hit handling in battle-actions.ts
    pub fn get_multi_hit_count(multi_hit: Option<i32>, random_value: i32) -> i32 {
        match multi_hit {
            None => 1,
            Some(n) if n <= 1 => 1,
            Some(2) => 2,
            Some(3) => 3,
            Some(5) => {
                // 2-5 hits: 35% for 2, 35% for 3, 15% for 4, 15% for 5
                match random_value % 100 {
                    0..=34 => 2,
                    35..=69 => 3,
                    70..=84 => 4,
                    _ => 5,
                }
            }
            Some(n) => n,
        }
    }
}
