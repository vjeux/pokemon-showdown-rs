//! Confusion Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onStart
/// TODO: Implement 1-to-1 from JavaScript
/// JavaScript source (data/conditions.ts):
/// confusion: {
///     onStart(...) {
///         // Extract implementation from conditions.ts
///     }
/// }
pub fn on_start(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[CONFUSION_ON_START] Called for {:?}", pokemon_pos);
    // TODO: Implement callback
    EventResult::Continue
}

/// onEnd
/// TODO: Implement 1-to-1 from JavaScript
/// JavaScript source (data/conditions.ts):
/// confusion: {
///     onEnd(...) {
///         // Extract implementation from conditions.ts
///     }
/// }
pub fn on_end(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[CONFUSION_ON_END] Called for {:?}", pokemon_pos);
    // TODO: Implement callback
    EventResult::Continue
}

/// onBeforeMove
/// TODO: Implement 1-to-1 from JavaScript
/// JavaScript source (data/conditions.ts):
/// confusion: {
///     onBeforeMove(...) {
///         // Extract implementation from conditions.ts
///     }
/// }
pub fn on_before_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[CONFUSION_ON_BEFORE_MOVE] Called for {:?}", pokemon_pos);
    // TODO: Implement callback
    EventResult::Continue
}

