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
        // CRITICAL: Match JavaScript's floating-point behavior exactly.
        // In JavaScript, some multiplications like 10 * 1.1 = 11 exactly (integer),
        // while others like 50 * 1.1 = 55.00000000000001 (non-integer).
        // The modifier (from chainModify) is only applied if relayVar === Math.floor(relayVar).
        // So we need to check if the result is an exact integer:
        // - If exact integer → return Number (modifier WILL be applied)
        // - If non-integer → return Float (modifier will NOT be applied)
        let result = base_power as f64 * 1.1;
        let floored = result.floor();
        if result == floored {
            // Exact integer - return as Number so modifier can be applied
            return EventResult::Number(floored as i32);
        } else {
            // Non-integer - return as Float to prevent modifier application
            return EventResult::Float(result);
        }
    }

    EventResult::Continue
}
