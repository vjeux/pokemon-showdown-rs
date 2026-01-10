//! DamageResult enum for battle action damage calculation results

/// Damage calculation result
/// Matches JavaScript return types: number | undefined | false | true | '' (NOT_FAIL) | HIT_SUBSTITUTE
/// JavaScript functions return number (damage) | undefined (no effect) | false (failed) | true (success) | '' (NOT_FAIL)
///
/// In JavaScript Pokemon Showdown:
/// - number: Actual damage dealt
/// - false: Move explicitly failed (shows "But it failed!" message)
/// - true: Move succeeded without dealing damage
/// - undefined: Move had no effect/immune (no failure message)
/// - '' (NOT_FAIL): Move didn't fail but also didn't succeed normally (e.g., protected)
/// - HIT_SUBSTITUTE: Substitute took the hit (treated as 0 damage)
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum DamageResult {
    /// Actual damage dealt (number in JS)
    /// JavaScript: return 123
    Damage(i32),

    /// Move explicitly failed (false in JS)
    /// Shows "But it failed!" or similar message
    /// JavaScript: return false
    Failed,

    /// Move succeeded without damage (true in JS)
    /// JavaScript: return true
    Success,

    /// No effect/undefined (undefined in JS)
    /// Target immune, move had no effect, or similar
    /// Does NOT show a failure message
    /// JavaScript: return undefined
    #[default]
    Undefined,

    /// NOT_FAIL - didn't fail but also didn't succeed normally ('' in JS)
    /// Used when move was blocked (e.g., by Protect) but shouldn't show failure message
    /// JavaScript: return this.battle.NOT_FAIL (which is '')
    NotFail,

    /// Substitute took the hit (HIT_SUBSTITUTE constant in JS)
    /// Treated as 0 damage - substitute absorbed the attack
    /// JavaScript: return this.battle.HIT_SUBSTITUTE
    HitSubstitute,
}

/// Convert EventResult to DamageResult for move effect handling
impl From<crate::event::EventResult> for DamageResult {
    fn from(event_result: crate::event::EventResult) -> Self {
        use crate::event::EventResult;
        match event_result {
            EventResult::Number(n) => DamageResult::Damage(n),
            EventResult::Boolean(true) | EventResult::Stop => DamageResult::Success,
            EventResult::Boolean(false) => DamageResult::Failed,
            EventResult::NotFail => DamageResult::NotFail,
            EventResult::Null => DamageResult::Undefined,
            EventResult::Continue => DamageResult::Success,
            _ => DamageResult::Undefined,
        }
    }
}

impl DamageResult {
    /// Check if the result represents actual damage
    pub fn is_damage(&self) -> bool {
        matches!(self, DamageResult::Damage(_))
    }

    /// Check if the result represents explicit failure
    pub fn is_failed(&self) -> bool {
        matches!(self, DamageResult::Failed)
    }

    /// Check if the result is NOT_FAIL
    pub fn is_not_fail(&self) -> bool {
        matches!(self, DamageResult::NotFail)
    }

    /// Check if the result is undefined
    pub fn is_undefined(&self) -> bool {
        matches!(self, DamageResult::Undefined)
    }

    /// Check if the result is Success
    pub fn is_success_flag(&self) -> bool {
        matches!(self, DamageResult::Success)
    }

    /// Check if the result is HitSubstitute
    pub fn is_hit_substitute(&self) -> bool {
        matches!(self, DamageResult::HitSubstitute)
    }

    /// Check if the result represents a truthy success (Damage > 0, Success, or HitSubstitute)
    /// In JavaScript: if (result) checks truthy, so 0 is falsy, positive numbers are truthy
    pub fn is_success(&self) -> bool {
        match self {
            DamageResult::Damage(d) => *d > 0,
            DamageResult::Success => true,
            DamageResult::HitSubstitute => true,
            _ => false,
        }
    }

    /// Get the damage value if it's a Damage result, otherwise 0
    pub fn damage_or_zero(&self) -> i32 {
        match self {
            DamageResult::Damage(d) => *d,
            _ => 0,
        }
    }

    /// Get the damage value if it's a Damage result
    pub fn damage(&self) -> Option<i32> {
        match self {
            DamageResult::Damage(d) => Some(*d),
            _ => None,
        }
    }
}
