//! Futuremove Condition
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
/// futuremove: {
///     onStart(...) {
///         // Extract implementation from conditions.ts
///     }
/// }
pub fn on_start(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[FUTUREMOVE_ON_START] Called for {:?}", pokemon_pos);
    // TODO: Implement callback
    EventResult::Continue
}

/// onResidual
/// TODO: Implement 1-to-1 from JavaScript
/// JavaScript source (data/conditions.ts):
/// futuremove: {
///     onResidual(...) {
///         // Extract implementation from conditions.ts
///     }
/// }
pub fn on_residual(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[FUTUREMOVE_ON_RESIDUAL] Called for {:?}", pokemon_pos);
    // TODO: Implement callback
    EventResult::Continue
}

/// onEnd
/// TODO: Implement 1-to-1 from JavaScript
/// JavaScript source (data/conditions.ts):
/// futuremove: {
///     onEnd(...) {
///         // Extract implementation from conditions.ts
///     }
/// }
pub fn on_end(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[FUTUREMOVE_ON_END] Called for {:?}", pokemon_pos);
    // TODO: Implement callback
    EventResult::Continue
}

