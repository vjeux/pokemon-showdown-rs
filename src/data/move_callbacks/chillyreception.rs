//! Chilly Reception Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// priorityChargeCallback(source) {
///     source.addVolatile('chillyreception');
/// }
pub fn priority_charge_callback(battle: &mut Battle, source_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onBeforeMove(source, target, move) {
    ///     if (move.id !== 'chillyreception') return;
    ///     this.add('-prepare', source, 'Chilly Reception', '[premajor]');
    /// }
    pub fn on_before_move(battle: &mut Battle, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

}
