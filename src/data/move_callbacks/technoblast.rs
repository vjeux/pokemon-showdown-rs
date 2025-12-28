//! Techno Blast Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyType(move, pokemon) {
///     if (pokemon.ignoringItem()) return;
///     move.type = this.runEvent('Drive', pokemon, null, move, 'Normal');
/// }
pub fn on_modify_type(
    _battle: &mut Battle,
    _move_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}
