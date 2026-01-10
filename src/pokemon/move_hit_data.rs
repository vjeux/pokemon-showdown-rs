//! Move hit data for tracking crit, type effectiveness, etc.

use serde::{Deserialize, Serialize};

/// Move hit data for tracking crit, type effectiveness, etc.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// JavaScript equivalent: inline tracking object (sim/battle-actions.ts)
/// 5 fields in JavaScript (used in damage calculation and protection tracking)
pub struct MoveHitData {
    /// Was this hit a critical hit?
    /// JavaScript: crit: boolean
    pub crit: bool,
    /// Type effectiveness modifier (-2 to 2, for 0.25x to 4x)
    /// JavaScript: typeMod: number
    pub type_mod: i8,
    /// Actual damage dealt
    /// JavaScript: damage: number
    pub damage: i32,
    /// Did the move hit the substitute instead?
    /// JavaScript: hitSubstitute: boolean
    pub hit_substitute: bool,
    /// Did this Z/Max move break through protection?
    /// JavaScript: zBrokeProtect: boolean
    pub z_broke_protect: bool,
}
