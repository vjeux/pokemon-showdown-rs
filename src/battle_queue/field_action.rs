//! Field action types

use serde::{Deserialize, Serialize};

/// Field action choice type
/// JavaScript equivalent: FieldAction.choice type (sim/battle-queue.ts)
/// JavaScript: 'start' | 'residual' | 'pass' | 'beforeTurn'
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldActionType {
    Start,
    Residual,
    Pass,
    BeforeTurn,
}

/// Field action (not done by a pokemon)
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: FieldAction (sim/battle-queue.ts)
/// 4 fields in JavaScript
pub struct FieldAction {
    /// Action type
    /// JavaScript: choice: 'start' | 'residual' | 'pass' | 'beforeTurn'
    pub choice: FieldActionType,
    /// Priority
    /// JavaScript: priority: number
    pub priority: i8,
    // TODO: DELETE - Not in JavaScript FieldAction (Rust-specific for tie-breaking)
    /// Sub-order for tie-breaking (lower = earlier)
    pub sub_order: i32,
    // TODO: DELETE - Not in JavaScript FieldAction (Rust-specific for tie-breaking)
    /// Effect order for tie-breaking (lower = earlier)
    pub effect_order: i32,
    // Note: JavaScript has 'speed' and 'pokemon' fields, but FieldActions don't have a pokemon
    // JavaScript: speed is not used in Rust implementation
    // JavaScript: pokemon is undefined for field actions
}
