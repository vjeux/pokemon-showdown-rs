use crate::*;

impl<'a> BattleActions<'a> {

    /// Spread move hit - apply to all targets
    /// Equivalent to spreadMoveHit in battle-actions.ts
    pub fn spread_move_hit_modifier(target_count: usize) -> f64 {
        Self::get_spread_damage_modifier(target_count)
    }
}
