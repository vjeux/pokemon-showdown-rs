//! Twisted Spoon Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, user, target, move) {
///     if (move.type === 'Psychic') {
///         return this.chainModify([4915, 4096]);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, _base_power: i32, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // Get the active move
    let active_move = match &battle.active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // if (move.type === 'Psychic') {
    //     return this.chainModify([4915, 4096]);
    // }
    if active_move.move_type == "Psychic" {
        let result = battle.chain_modify_fraction(4915, 4096);
        return EventResult::Number(result);
    }

    EventResult::Continue
}
