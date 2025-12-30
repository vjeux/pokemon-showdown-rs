use crate::*;

impl<'a> BattleActions<'a> {

    /// Try move hit - main hit attempt
    /// Equivalent to tryMoveHit in battle-actions.ts
    pub fn try_move_hit_check(
        accuracy_passed: bool,
        type_immunity_passed: bool,
        invulnerability_passed: bool,
    ) -> bool {
        accuracy_passed && type_immunity_passed && invulnerability_passed
    }
}
