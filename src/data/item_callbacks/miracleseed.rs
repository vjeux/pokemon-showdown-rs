//! Miracle Seed Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onBasePower(basePower, user, target, move) {
///     if (move.type === 'Grass') {
///         return this.chainModify([4915, 4096]);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, _base_power: i32, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // if (move.type === 'Grass') {
    //     return this.chainModify([4915, 4096]);
    // }
    let move_type = match &battle.active_move {
        Some(active_move) => &active_move.move_type,
        None => return EventResult::Continue,
    };

    if move_type == "Grass" {
        battle.chain_modify_fraction(4915, 4096);
    }

    EventResult::Continue
}
