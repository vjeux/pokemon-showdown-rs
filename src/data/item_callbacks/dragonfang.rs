//! Dragon Fang Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, user, target, move) {
///     if (move && move.type === 'Dragon') {
///         return this.chainModify([4915, 4096]);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, _base_power: i32, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // Get the active move
    let move_type = match &battle.active_move {
        Some(active_move) => {
            // Get the move data to check its type
            match battle.dex.get_move_by_id(&active_move.id) {
                Some(move_data) => &move_data.move_type,
                None => return EventResult::Continue,
            }
        }
        None => return EventResult::Continue,
    };

    // if (move && move.type === 'Dragon')
    if move_type == "Dragon" {
        // return this.chainModify([4915, 4096]);
        return EventResult::Number(battle.chain_modify_fraction(4915, 4096));
    }

    EventResult::Continue
}
