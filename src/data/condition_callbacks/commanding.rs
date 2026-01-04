//! Commanding Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onDragOut
/// TODO: Implement 1-to-1 from JavaScript
/// JavaScript source (data/conditions.ts):
/// commanding: {
///     onDragOut(...) {
///         // Extract implementation from conditions.ts
///     }
/// }
pub fn on_drag_out(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[COMMANDING_ON_DRAG_OUT] Called for {:?}", pokemon_pos);
    // TODO: Implement callback
    EventResult::Continue
}

/// onDragOutPriority
/// TODO: Implement 1-to-1 from JavaScript
/// JavaScript source (data/conditions.ts):
/// commanding: {
///     onDragOutPriority(...) {
///         // Extract implementation from conditions.ts
///     }
/// }
pub fn on_drag_out_priority(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[COMMANDING_ON_DRAG_OUT_PRIORITY] Called for {:?}", pokemon_pos);
    // TODO: Implement callback
    EventResult::Continue
}

