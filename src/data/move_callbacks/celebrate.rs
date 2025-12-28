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
pub fn on_try_hit(battle: &mut Battle, _source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    // this.add('-activate', target, 'move: Celebrate');
    let target_ident = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.get_slot()
    };

    battle.add("-activate", &[target_ident.as_str().into(), "move: Celebrate".into()]);

    EventResult::Continue
}
