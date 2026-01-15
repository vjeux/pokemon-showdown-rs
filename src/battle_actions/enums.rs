//! Battle action enums

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Ignore immunity setting for moves
/// JavaScript: ignoreImmunity?: boolean | { [k: string]: boolean }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IgnoreImmunity {
    /// Ignore all type immunities (true)
    All,
    /// Ignore specific type immunities ({ Type: true, ... })
    Specific(HashMap<String, bool>),
    /// Explicitly don't ignore immunities (false) - different from undefined
    /// This is used for moves like Thunder Wave that have ignoreImmunity: false
    NoIgnore,
}

/// Fixed damage value for moves
/// JavaScript: damage?: number | 'level' | false | null
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Damage {
    /// Fixed numeric damage
    Fixed(i32),
    /// Damage equals user's level (Seismic Toss, Night Shade)
    Level,
}

/// Damage value (can be number, false, or undefined-like None)
/// JavaScript: number | false | undefined
#[derive(Debug, Clone)]
pub enum DamageValue {
    Damage(i32),
    Failed,
    HitSubstitute,
}

/// Switch copy flag type
/// JavaScript uses string literals 'copyvolatile' | 'shedtail' | true
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchCopyFlag {
    None,
    CopyVolatile,
    ShedTail,
}

/// Target info for spread moves (can be the Pokemon or null/false for failed)
/// JavaScript: Pokemon | null | false
#[derive(Debug, Clone)]
pub enum SpreadMoveTarget {
    Target((usize, usize)),
    None,
    Failed,
}

/// Hit result for move steps (tryHit, etc.)
/// JavaScript: hitResults is (number | boolean | "" | undefined)[]
/// - true: hit succeeded
/// - false: explicitly failed (counts as atLeastOneFailure)
/// - "" (NOT_FAIL): blocked but not a failure (doesn't count as atLeastOneFailure)
///
/// This distinction is critical for moves like Temper Flare that check
/// `pokemon.moveLastTurnResult === false` - a move blocked by Protect
/// should NOT be considered a failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitResult {
    /// Hit succeeded (truthy in JavaScript)
    Success,
    /// Explicitly failed (counts as atLeastOneFailure)
    Failed,
    /// Blocked but NOT a failure - e.g., blocked by Protect/Max Guard
    /// This is JavaScript's NOT_FAIL ("")
    NotFail,
}

/// Move result state for moveThisTurnResult/moveLastTurnResult
/// JavaScript: moveThisTurnResult?: boolean | null | undefined
///
/// In JavaScript:
/// - undefined: never set (initial state)
/// - null: explicitly set to null (not a failure, just no result)
/// - true: move succeeded
/// - false: move failed
///
/// The distinction between undefined and null is critical because:
/// `undefined === null` is false in JavaScript, so the comparison
/// `oldMoveResult === pokemon.moveThisTurnResult` works correctly.
///
/// This is important for Temper Flare which checks
/// `pokemon.moveLastTurnResult === false`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum MoveResult {
    /// JavaScript undefined - never set or reset to initial state
    #[default]
    Undefined,
    /// JavaScript null - explicitly set to null (move blocked but not "failed")
    Null,
    /// JavaScript true - move succeeded
    Success,
    /// JavaScript false - move explicitly failed
    Failed,
}

impl MoveResult {
    /// Check if the result equals JavaScript false (for Temper Flare check)
    pub fn is_false(&self) -> bool {
        matches!(self, MoveResult::Failed)
    }
}
