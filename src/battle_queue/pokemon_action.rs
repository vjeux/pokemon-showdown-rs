//! Pokemon action types

use serde::{Deserialize, Serialize};

/// Pokemon action choice type
/// JavaScript equivalent: PokemonAction.choice type (sim/battle-queue.ts)
/// JavaScript: 'start' | 'beforeTurn' | 'megaEvo' | 'megaEvoX' | 'megaEvoY' | 'shift' | 'runSwitch' | 'event' | 'runDynamax' | 'terastallize' | 'residual'
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PokemonActionType {
    Start,
    BeforeTurn,
    MegaEvo,
    MegaEvoX,
    MegaEvoY,
    Shift,
    RunSwitch,
    Event,
    RunDynamax,
    Terastallize,
    Residual,
}

/// Pokemon action (misc actions by a single pokemon)
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: PokemonAction (sim/battle-queue.ts)
/// 6 fields in JavaScript
pub struct PokemonAction {
    /// Action type
    /// JavaScript: choice: 'start' | 'beforeTurn' | 'megaEvo' | 'megaEvoX' | 'megaEvoY' | 'shift' | 'runSwitch' | 'event' | 'runDynamax' | 'terastallize' | 'residual'
    pub choice: PokemonActionType,
    // TODO: DELETE - Not in JavaScript PokemonAction (Rust-specific)
    /// Order for sorting
    pub order: i32,
    /// Priority
    /// JavaScript: priority: number
    pub priority: i8,
    /// Speed
    /// JavaScript: speed: number
    pub speed: f64,
    // TODO: DELETE - Not in JavaScript PokemonAction (Rust-specific for tie-breaking)
    /// Sub-order for tie-breaking (lower = earlier)
    pub sub_order: i32,
    // TODO: DELETE - Not in JavaScript PokemonAction (Rust-specific for tie-breaking)
    /// Effect order for tie-breaking (lower = earlier)
    pub effect_order: i32,
    /// Pokemon index
    /// JavaScript: pokemon: Pokemon
    /// TODO: Rust uses index instead of Pokemon reference due to ownership
    pub pokemon_index: usize,
    // TODO: DELETE - Not in JavaScript PokemonAction (Rust-specific)
    /// Side index
    pub side_index: usize,
    /// Event name (for event actions)
    /// JavaScript: event?: string
    pub event: Option<String>,
    /// Pokemon that is dragging this pokemon (for Red Card, Roar, etc.)
    /// JavaScript: dragger?: Pokemon
    /// TODO: Rust uses indices instead of Pokemon reference due to ownership
    pub dragger: Option<(usize, usize)>,
}
