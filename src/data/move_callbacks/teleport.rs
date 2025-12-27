//! Teleport Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry(source) {
///     return !!this.canSwitch(source.side);
/// }
pub fn on_try(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // Check if source's side can switch
    let can_switch = battle.can_switch(source_pos.0);
    EventResult::Bool(can_switch)
}

