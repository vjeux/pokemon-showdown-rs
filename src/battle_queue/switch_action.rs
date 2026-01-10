//! Switch action types

use serde::{Deserialize, Serialize};

use crate::battle::Effect;

/// Switch action choice type
/// JavaScript equivalent: SwitchAction.choice type (sim/battle-queue.ts)
/// JavaScript: 'switch' | 'instaswitch' | 'revivalblessing'
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SwitchActionType {
    Switch,
    InstaSwitch,
    RevivalBlessing,
}

/// Switch action
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: SwitchAction (sim/battle-queue.ts)
/// 7 fields in JavaScript
pub struct SwitchAction {
    /// Action type
    /// JavaScript: choice: 'switch' | 'instaswitch' | 'revivalblessing'
    pub choice: SwitchActionType,
    /// Order for sorting
    pub order: i32,
    /// Priority of the action
    pub priority: i8,
    /// Speed of pokemon switching
    pub speed: f64,
    // TODO: DELETE - Not in JavaScript SwitchAction (Rust-specific for tie-breaking)
    /// Sub-order for tie-breaking (lower = earlier)
    pub sub_order: i32,
    // TODO: DELETE - Not in JavaScript SwitchAction (Rust-specific for tie-breaking)
    /// Effect order for tie-breaking (lower = earlier)
    pub effect_order: i32,
    /// Index of the pokemon doing the switch
    /// JavaScript: pokemon: Pokemon
    /// TODO: Rust uses indices instead of Pokemon reference due to ownership
    pub pokemon_index: usize,
    // TODO: DELETE - Not in JavaScript SwitchAction (Rust-specific)
    /// Side index of the pokemon
    pub side_index: usize,
    /// Index of pokemon to switch to
    /// JavaScript: target: Pokemon
    /// TODO: Rust uses indices instead of Pokemon reference due to ownership
    pub target_index: usize,
    /// Effect that caused the switch
    /// JavaScript: sourceEffect: Effect | null
    pub source_effect: Option<Effect>,
}
