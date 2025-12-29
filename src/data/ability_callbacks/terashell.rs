//! Tera Shell Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAnyBeforeMove() {
///     delete this.effectState.resisted;
/// }
pub fn on_any_before_move(battle: &mut Battle) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onAnyAfterMove() {
///     delete this.effectState.resisted;
/// }
pub fn on_any_after_move(battle: &mut Battle) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

