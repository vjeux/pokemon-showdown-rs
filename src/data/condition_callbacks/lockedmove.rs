//! Lockedmove Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onResidual
/// TODO: Implement 1-to-1 from JavaScript
/// JavaScript source (data/conditions.ts):
/// lockedmove: {
///     onResidual(...) {
///         // Extract implementation from conditions.ts
///     }
/// }
pub fn on_residual(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[LOCKEDMOVE_ON_RESIDUAL] Called for {:?}", pokemon_pos);
    // TODO: Implement callback
    EventResult::Continue
}

