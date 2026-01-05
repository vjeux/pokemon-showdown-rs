//! Throat Spray Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onAfterMoveSecondarySelf(target, source, move) {
///     if (move.flags['sound']) {
///         target.useItem();
///     }
/// }
pub fn on_after_move_secondary_self(battle: &mut Battle, source_pos: (usize, usize), _target_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // if (move.flags['sound']) {
    //     target.useItem();
    // }

    // Check if move has sound flag
    let is_sound_move = match &battle.active_move {
        Some(active_move) => active_move.flags.sound,
        None => return EventResult::Continue,
    };

    if is_sound_move {
        // target.useItem();
        let _pokemon_mut = match battle.pokemon_at_mut(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        Pokemon::use_item(battle, source_pos, None, None);
    }

    EventResult::Continue
}
