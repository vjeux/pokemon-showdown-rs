//! Defense Curl Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

// Condition handlers
pub mod condition {
    use super::*;

    /// onRestart() {
    ///     return null;
    /// }
    pub fn on_restart(
        _battle: &mut Battle,
        _pokemon_pos: (usize, usize),
    ) -> EventResult {
        // return null; - prevents the volatile from being restarted
        EventResult::Null
    }
}
