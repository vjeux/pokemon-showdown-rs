//! Celebrate Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::event::EventResult;

/// onTryHit(target, source) {
///     this.add('-activate', target, 'move: Celebrate');
/// }
pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    // this.add('-activate', target, 'move: Celebrate');
    let target_arg = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        Arg::from(target)
    };

    battle.add("-activate", &[target_arg, "move: Celebrate".into()]);

    EventResult::Continue
}
