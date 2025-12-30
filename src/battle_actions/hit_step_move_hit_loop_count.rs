use crate::*;

impl<'a> BattleActions<'a> {

    /// Hit step move hit loop
    /// Equivalent to hitStepMoveHitLoop in battle-actions.ts
    pub fn hit_step_move_hit_loop_count(
        multi_hit: Option<i32>,
        move_hit_type: Option<&str>,
        random_value: i32,
    ) -> i32 {
        if let Some(hit_type) = move_hit_type {
            match hit_type {
                "parentalbond" => 2,
                "triplekick" => 3,
                _ => {
                    // Inline logic from deleted get_multi_hit_count
                    // TODO: This function itself likely needs deletion - will be evaluated when reached in sequence
                    match multi_hit {
                        None => 1,
                        Some(n) if n <= 1 => 1,
                        Some(2) => 2,
                        Some(3) => 3,
                        Some(5) => {
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
        } else {
            // Inline logic from deleted get_multi_hit_count
            match multi_hit {
                None => 1,
                Some(n) if n <= 1 => 1,
                Some(2) => 2,
                Some(3) => 3,
                Some(5) => {
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
}
