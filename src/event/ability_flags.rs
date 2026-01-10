//! Ability Flags

use serde::{Deserialize, Serialize};

/// Flags that can be set on abilities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// JavaScript equivalent: AbilityFlags (sim/dex-abilities.ts)
/// 8 fields in JavaScript
pub struct AbilityFlags {
    /// Can be suppressed by Mold Breaker etc.
    /// JavaScript: breakable?: 1
    pub breakable: bool,
    /// Cannot be copied by Role Play, etc.
    /// JavaScript: noCopy?: 1
    pub no_copy: bool,
    /// Cannot be suppressed
    /// JavaScript: cannotSuppress?: 1
    pub cannot_suppress: bool,
    /// Cannot be traced
    /// JavaScript: noTrace?: 1
    pub no_trace: bool,
    /// Cannot be skill swapped
    /// JavaScript: noSkillSwap?: 1
    pub no_skill_swap: bool,
    /// Fails Entrainment
    /// JavaScript: failEntrain?: 1
    pub fail_entrainment: bool,
}
