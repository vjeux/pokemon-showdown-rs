// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl<'a> BattleActions<'a> {

    // =========================================================================
    // SWITCH METHODS - Ported from battle-actions.ts
    // =========================================================================
    // Note: switchIn, dragIn, runSwitch are implemented on Battle struct
    // because they need mutable access to battle state.

    // =========================================================================
    // MOVE METHODS - Ported from battle-actions.ts
    // =========================================================================
    // Note: runMove, useMove, useMoveInner are implemented on Battle struct
    // because they need mutable access to battle state.

    // =========================================================================
    // DAMAGE CALCULATION - These can be pure functions
    // =========================================================================

    /// Calculate the stat modifier from boost stages
    /// Returns the multiplier as a fraction (numerator, denominator)
    /// Equivalent to getBoostMod in Pokemon Showdown
    pub fn get_boost_modifier(boost: i8) -> (i32, i32) {
        match boost {
            -6 => (2, 8),
            -5 => (2, 7),
            -4 => (2, 6),
            -3 => (2, 5),
            -2 => (2, 4),
            -1 => (2, 3),
            0 => (2, 2),
            1 => (3, 2),
            2 => (4, 2),
            3 => (5, 2),
            4 => (6, 2),
            5 => (7, 2),
            6 => (8, 2),
            _ if boost < -6 => (2, 8),
            _ => (8, 2),
        }
    }
}
