//! Throat Spray Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;
use crate::Pokemon;

/// onAfterMoveSecondarySelf(target, source, move) {
///     if (move.flags['sound']) {
///         target.useItem();
///     }
/// }
pub fn on_after_move_secondary_self(battle: &mut Battle, source_pos: (usize, usize), _target_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // if (move.flags['sound']) {
    //     target.useItem();
    // }

    // Get active_move from parameter
    let active_move_ref = match active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // Check if move has sound flag
    if active_move_ref.flags.sound {
        // target.useItem();
        let _pokemon_mut = match battle.pokemon_at_mut(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        Pokemon::use_item(battle, source_pos, None, None);
    }

    EventResult::Continue
}
