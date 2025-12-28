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
    let target_arg = {
        let pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        crate::battle::Arg::from(pokemon)
    };
    battle.add("-activate", &[target_arg, "move: Happy Hour".into()]);

    EventResult::Continue
}

