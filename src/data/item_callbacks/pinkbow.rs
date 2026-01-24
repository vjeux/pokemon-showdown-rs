//! Pink Bow Item
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
        Some(active_move) => active_move.borrow().move_type.clone(),
        None => return EventResult::Continue,
    };

    // if (move.type === 'Normal')
    if move_type == "Normal" {
        // return basePower * 1.1;
        // CRITICAL: Return Float, not Number! In JS, this returns a non-integer (e.g., 19.8)
        // which fails the `relayVar === Math.floor(relayVar)` check in runEvent,
        // so the accumulated modifier is NOT applied to this result.
        return EventResult::Float(base_power as f64 * 1.1);
    }

    EventResult::Continue
}
