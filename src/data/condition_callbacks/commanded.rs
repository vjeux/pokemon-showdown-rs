//! Commanded Condition
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
/// commanded: {
///     onStart(...) {
///         // Extract implementation from conditions.ts
///     }
/// }
pub fn on_start(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[COMMANDED_ON_START] Called for {:?}", pokemon_pos);
    // TODO: Implement callback
    EventResult::Continue
}

