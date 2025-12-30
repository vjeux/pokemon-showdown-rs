use crate::*;

impl<'a> BattleActions<'a> {

    /// Check if move forces switch
    /// Equivalent to forceSwitch handling in battle-actions.ts
    pub fn should_force_switch(
        move_force_switch: bool,
        target_hp: i32,
        target_is_dynamaxed: bool,
    ) -> bool {
        if !move_force_switch {
            return false;
        }
        if target_hp == 0 {
            return false;
        }
        if target_is_dynamaxed {
            return false; // Dynamaxed Pokemon can't be forced out
        }
        true
    }
}
