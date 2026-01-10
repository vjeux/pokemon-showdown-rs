//! Move action types

use serde::{Deserialize, Serialize};

use crate::battle::Effect;
use crate::dex_data::ID;

/// Move action choice type
/// JavaScript equivalent: MoveAction.choice type (sim/battle-queue.ts)
/// JavaScript: 'move' | 'beforeTurnMove' | 'priorityChargeMove'
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoveActionType {
    Move,
    BeforeTurnMove,
    PriorityChargeMove,
}

/// Move action
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: MoveAction (sim/battle-queue.ts)
/// 14 fields in JavaScript
pub struct MoveAction {
    /// Action type
    /// JavaScript: choice: 'move' | 'beforeTurnMove' | 'priorityChargeMove'
    pub choice: MoveActionType,
    /// Order for sorting (lower = earlier)
    pub order: i32,
    /// Priority of the action (higher = earlier)
    pub priority: i8,
    /// Fractional priority (higher = earlier)
    /// JavaScript: fractionalPriority: number
    pub fractional_priority: f64,
    /// Speed of pokemon using move (higher = earlier if priority tie)
    pub speed: f64,
    // TODO: DELETE - Not in JavaScript MoveAction (Rust-specific for tie-breaking)
    /// Sub-order for tie-breaking (lower = earlier)
    pub sub_order: i32,
    // TODO: DELETE - Not in JavaScript MoveAction (Rust-specific for tie-breaking)
    /// Effect order for tie-breaking (lower = earlier)
    pub effect_order: i32,
    /// Index of the pokemon doing the move
    /// JavaScript: pokemon: Pokemon
    /// TODO: Rust uses indices instead of Pokemon reference due to ownership
    pub pokemon_index: usize,
    /// Side index of the pokemon
    /// TODO: Rust-specific - JavaScript has pokemon reference
    pub side_index: usize,
    /// Location of the target, relative to pokemon's side
    /// JavaScript: targetLoc: number
    pub target_loc: i8,
    /// Original target Pokemon
    /// JavaScript: originalTarget: Pokemon
    /// TODO: Rust uses indices instead of Pokemon reference due to ownership
    pub original_target: Option<(usize, usize)>,
    /// Move ID
    /// JavaScript: moveid: ID
    pub move_id: ID,
    /// True if mega evolving, or 'done' if already mega evolved
    /// JavaScript: mega: boolean | 'done'
    /// TODO: Rust uses bool, cannot represent 'done' variant
    pub mega: bool,
    /// Z-move name if using Z-move
    /// JavaScript: zmove?: string
    pub zmove: Option<String>,
    /// Max move name if dynamaxed
    /// JavaScript: maxMove?: string
    pub max_move: Option<String>,
    /// Source effect that triggered this action
    /// JavaScript: sourceEffect?: Effect | null
    pub source_effect: Option<Effect>,
    /// Tera type if terastallizing (Gen 9+)
    /// JavaScript: terastallize?: string
    pub terastallize: Option<String>,
    // TODO: DELETE - Not in JavaScript MoveAction
    /// Modified move priority for Quick Guard detection (Gen 6+)
    /// JavaScript: action.move.priority = priority
    /// Stores the priority value assigned to the move itself, allowing Quick Guard
    /// to detect if the move's priority was artificially enhanced (e.g., by Prankster)
    pub move_priority_modified: Option<i8>,
    /// True if this move's priority was boosted by Prankster
    /// JavaScript: move.pranksterBoosted: boolean
    /// Used to prevent Prankster-boosted moves from affecting Dark-type Pokemon
    pub prankster_boosted: bool,
}

impl MoveAction {
    /// Get move data from Dex
    /// Equivalent to accessing action.move in TypeScript
    /// Returns MoveData for this action's move
    pub fn get_move<'a>(&self, dex: &'a crate::dex::Dex) -> Option<&'a crate::dex::MoveData> {
        dex.moves().get(self.move_id.as_str())
    }
}
