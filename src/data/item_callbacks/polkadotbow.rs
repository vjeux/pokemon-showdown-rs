//! Polkadot Bow Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, user, target, move) {
///     if (move.type === 'Normal') {
///         return basePower * 1.1;
///     }
/// }
pub fn on_base_power(battle: &mut Battle, base_power: i32, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // Get the active move
    let move_type = match &battle.active_move {
        Some(active_move) => active_move.move_type.clone(),
        None => return EventResult::Continue,
    };

    // if (move.type === 'Normal')
    if move_type == "Normal" {
        // return basePower * 1.1;
        // CRITICAL: Return Float to match JavaScript behavior where basePower * 1.1
        // produces a non-integer (e.g., 55.00000000000001), which causes the
        // modifier (e.g., from Rivalry's chainModify) to NOT be applied.
        return EventResult::Float(base_power as f64 * 1.1);
    }

    EventResult::Continue
}
