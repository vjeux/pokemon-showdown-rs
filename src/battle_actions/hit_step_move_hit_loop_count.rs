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
                _ => Self::get_multi_hit_count(multi_hit, random_value),
            }
        } else {
            Self::get_multi_hit_count(multi_hit, random_value)
        }
    }
}
