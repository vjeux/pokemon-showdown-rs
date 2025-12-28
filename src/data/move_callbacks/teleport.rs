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
pub fn on_try(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    let source = source_pos;

    // return !!this.canSwitch(source.side);
    let can_switch = battle.can_switch(source.0);

    if can_switch > 0 {
        EventResult::Continue
    } else {
        EventResult::NotFail
    }
}
