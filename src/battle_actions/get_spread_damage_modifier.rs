use crate::*;

impl<'a> BattleActions<'a> {

    /// Calculate spread move damage modifier
    /// Equivalent to getSpreadDamage calculation in battle-actions.ts
    pub fn get_spread_damage_modifier(target_count: usize) -> f64 {
        if target_count > 1 {
            0.75 // 75% damage when hitting multiple targets
        } else {
            1.0
        }
    }
}
