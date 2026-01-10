//! Option structs for battle action functions

use crate::battle::Effect;
use crate::dex_data::BoostsTable;

/// Result from after move secondary event
#[derive(Debug, Clone, Default)]
pub struct AfterMoveResult {
    /// Self-switch effect type
    pub self_switch: Option<String>,
    /// Force switch flag
    pub force_switch: bool,
}

/// Move effects to apply
#[derive(Debug, Clone, Default)]
pub struct MoveEffects {
    /// Stat boosts to apply
    pub boosts: Option<BoostsTable>,
    /// Healing fraction
    pub heal: Option<(i32, i32)>,
    /// Status condition to inflict
    pub status: Option<String>,
    /// Volatile status to inflict
    pub volatile_status: Option<String>,
}

/// Run move options for runMove
#[derive(Debug, Clone, Default)]
pub struct RunMoveOptions {
    /// Source effect that caused this move
    pub source_effect: Option<Effect>,
    /// Z-move override
    pub z_move: Option<String>,
    /// External move (Dancer, etc.)
    pub external_move: bool,
    /// Max move override
    pub max_move: Option<String>,
    /// Original target for redirection tracking
    pub original_target: Option<usize>,
}

/// Use move options for useMove
#[derive(Debug, Clone, Default)]
pub struct UseMoveOptions {
    /// Target pokemon index
    pub target: Option<usize>,
    /// Source effect
    pub source_effect: Option<Effect>,
    /// Z-move override
    pub z_move: Option<String>,
    /// Max move override
    pub max_move: Option<String>,
}
