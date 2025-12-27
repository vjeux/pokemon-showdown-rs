//! Happy Hour Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source) {
///     this.add('-activate', target, 'move: Happy Hour');
/// }
pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    let target = target_pos;

    // this.add('-activate', target, 'move: Happy Hour');
    let target_arg = crate::battle::Arg::Pos(target.0, target.1);
    battle.add("-activate", &[target_arg, "move: Happy Hour".into()]);

    EventResult::Continue
}

