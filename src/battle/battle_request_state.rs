//! Battle Request State

use serde::{Deserialize, Serialize};

/// Request state for the whole battle
/// TODO: Not in JavaScript - Rust-specific enum for tracking battle request state
/// JavaScript uses string literals for request types ('move' | 'switch' | 'teampreview' | null)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BattleRequestState {
    #[default]
    None,
    TeamPreview,
    Move,
    Switch,
}
