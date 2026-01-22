//! Lagging Tail Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Makes the holder move last in its priority bracket.

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onFractionalPriority: -0.1
/// Static value that makes the holder move last in its priority bracket
pub fn on_fractional_priority(
    _battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _priority: f64,
) -> EventResult {
    // Return -0.1 to make the holder move last
    EventResult::Float(-0.1)
}
