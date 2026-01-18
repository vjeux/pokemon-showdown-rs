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
/// NOTE: dispatch_on_try_hit passes (target_pos, source_pos) per JS convention
pub fn on_try_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),
    _source_pos: (usize, usize),
) -> EventResult {
    // this.add('-activate', target, 'move: Celebrate');
    let target_ident = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.get_slot()
    };

    battle.add(
        "-activate",
        &[target_ident.as_str().into(), "move: Celebrate".into()],
    );

    EventResult::Continue
}
