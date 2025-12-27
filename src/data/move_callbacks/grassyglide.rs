//! Grassy Glide Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyPriority(priority, source, target, move) {
///     if (this.field.isTerrain('grassyterrain') && source.isGrounded()) {
///         return priority + 1;
///     }
/// }
pub fn on_modify_priority(battle: &mut Battle, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

