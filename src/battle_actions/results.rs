//! Result types for battle actions

use crate::dex_data::{BoostsTable, ID};

/// Result of run_move
#[derive(Debug, Clone)]
pub struct RunMoveResult {
    pub move_id: ID,
    pub pokemon_index: usize,
    pub target_loc: i32,
    pub z_move: Option<String>,
    pub max_move: Option<String>,
    pub external_move: bool,
    pub success: bool,
}

/// Result of run_z_power
#[derive(Debug, Clone)]
pub enum ZPowerResult {
    DamageMove,
    Boost(BoostsTable),
    Heal,
    HealReplacement,
    ClearNegativeBoost,
    Redirect,
    Crit2,
    None,
}

/// Result of terastallize
#[derive(Debug, Clone)]
pub enum TerastallizeResult {
    Success {
        tera_type: String,
        forme_change: Option<String>,
    },
    InvalidOgerpon,
}

/// Result of switchIn
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwitchInResult {
    /// Switch was successful
    Success,
    /// Switch was blocked (e.g., by an event returning false)
    Blocked,
    /// Pokemon fainted from Pursuit before switching
    PursuitFaint,
}
