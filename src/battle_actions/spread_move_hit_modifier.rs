use crate::*;

impl<'a> BattleActions<'a> {

    /// Spread move hit - apply to all targets
    /// Equivalent to spreadMoveHit in battle-actions.ts
    pub fn spread_move_hit_modifier(target_count: usize) -> f64 {
        // Inline logic from deleted get_spread_damage_modifier
        // TODO: This function itself likely needs deletion - will be evaluated when reached in sequence
        if target_count > 1 {
            0.75
        } else {
            1.0
        }
    }
}
