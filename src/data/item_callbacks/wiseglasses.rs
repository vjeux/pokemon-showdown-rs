//! Wise Glasses Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, user, target, move) {
///     if (move.category === 'Special') {
///         return this.chainModify([4505, 4096]);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, _base_power: i32, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // if (move.category === 'Special') {
    //     return this.chainModify([4505, 4096]);
    // }
    let category = match &battle.active_move {
        Some(active_move) => active_move.category.clone(),
        None => return EventResult::Continue,
    };

    if category == "Special" {
        // return this.chainModify([4505, 4096]);
        return EventResult::Number(battle.chain_modify_fraction(4505, 4096));
    }

    EventResult::Continue
}
