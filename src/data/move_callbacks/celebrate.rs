//! Celebrate Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source) {
///     this.add('-activate', target, 'move: Celebrate');
/// }
pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    use crate::battle::Arg;
    battle.add("-activate", &[Arg::Pokemon(target), Arg::Str("move: Celebrate")]);

    EventResult::Continue
}

