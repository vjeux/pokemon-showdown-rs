//! Move-related data structs

use serde::{Deserialize, Serialize};
use crate::dex_data::BoostsTable;

/// Move hit data for tracking crit, type effectiveness, etc.
#[derive(Debug, Clone, Default)]
/// JavaScript equivalent: MoveHitData (sim/dex-moves.ts)
pub struct MoveHitData {
    /// Critical hit flag
    pub crit: bool,
    /// Type effectiveness modifier
    pub type_mod: i32,
    /// Damage dealt (Rust-specific tracking)
    pub damage: i32,
    /// Z-Move broke through protect
    pub z_broke_protect: bool,
}

/// Max move data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// JavaScript equivalent: MaxMoveData (sim/dex-moves.ts)
pub struct MaxMoveData {
    /// Base power of the Max Move
    #[serde(rename = "basePower", default)]
    pub base_power: i32,
}

/// Z-move data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// JavaScript equivalent: ZMoveData (sim/dex-moves.ts)
pub struct ZMoveData {
    /// Base power of the Z-Move
    #[serde(rename = "basePower", default)]
    pub base_power: Option<i32>,
    /// Stat boosts from Z-Move
    #[serde(default)]
    pub boost: Option<BoostsTable>,
    /// Effect ID
    #[serde(default)]
    pub effect: Option<String>,
}

/// Z-Move request option
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: ZMoveOption (sim/side.ts)
pub struct ZMoveOption {
    pub move_name: String,
    pub target: String,
}
