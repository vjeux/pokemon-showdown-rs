use crate::*;
use crate::battle_actions::SecondaryEffect;

impl<'a> BattleActions<'a> {

    /// Get secondary effects for a move
    /// Equivalent to secondaries in battle-actions.ts
    pub fn get_secondaries(
        secondaries: &[SecondaryEffect],
        has_sheer_force: bool,
    ) -> Vec<SecondaryEffect> {
        if has_sheer_force {
            // Sheer Force removes secondary effects
            return Vec::new();
        }
        secondaries.to_vec()
    }
}
