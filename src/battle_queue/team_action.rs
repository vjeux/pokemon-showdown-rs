//! Team action types

use serde::{Deserialize, Serialize};

/// Team action choice type
/// JavaScript equivalent: TeamAction.choice type (sim/battle-queue.ts)
/// JavaScript: 'team'
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TeamActionType {
    Team,
}

/// Team preview choice action
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: TeamAction (sim/battle-queue.ts)
/// 5 fields in JavaScript
pub struct TeamAction {
    /// Action type (always 'team' in JavaScript)
    /// JavaScript: choice: 'team'
    pub choice: TeamActionType,
    /// Priority (negative index for team actions)
    pub priority: i8,
    /// Speed of pokemon (for tie-breaking)
    /// JavaScript: speed: 1
    pub speed: f64,
    // TODO: DELETE - Not in JavaScript TeamAction (Rust-specific for tie-breaking)
    /// Sub-order for tie-breaking (lower = earlier)
    pub sub_order: i32,
    // TODO: DELETE - Not in JavaScript TeamAction (Rust-specific for tie-breaking)
    /// Effect order for tie-breaking (lower = earlier)
    pub effect_order: i32,
    /// Pokemon index
    /// JavaScript: pokemon: Pokemon
    /// TODO: Rust uses index instead of Pokemon reference due to ownership
    pub pokemon_index: usize,
    // TODO: DELETE - Not in JavaScript TeamAction (Rust-specific)
    /// Side index
    pub side_index: usize,
    /// New index in team order
    pub index: usize,
}
