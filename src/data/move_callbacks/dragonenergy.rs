//! Dragon Energy Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target, move) {
///     const bp = move.basePower * pokemon.hp / pokemon.maxhp;
///     this.debug(`BP: ${bp}`);
///     return bp;
/// }
pub fn base_power_callback(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    let pokemon = pokemon_pos;

    // Get current move's base power
    let base_power = match &battle.active_move {
        Some(active_move) => active_move.base_power,
        None => return EventResult::Continue,
    };

    // const bp = move.basePower * pokemon.hp / pokemon.maxhp;
    // JavaScript uses floating point. The callback returns the float (e.g., 0.625),
    // which is truthy so it passes the `if (!basePower) return 0;` check.
    // Later, clampIntRange(basePower, 1) truncates to 0 and maxes to 1.
    //
    // In Rust, we need to return i32, so we calculate the float and apply the
    // equivalent of clampIntRange: truncate to integer, then max with 1 if the
    // original float was > 0.
    let bp = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let bp_float = base_power as f64 * pokemon_pokemon.hp as f64 / pokemon_pokemon.maxhp as f64;
        let bp_truncated = bp_float as i32;
        // Match JavaScript behavior: if bp_float > 0 but truncates to 0, return 1
        // This is equivalent to clampIntRange which comes later in JS
        if bp_truncated == 0 && bp_float > 0.0 {
            1
        } else {
            bp_truncated
        }
    };

    // this.debug(`BP: ${bp}`);
    battle.debug(&format!("BP: {}", bp));

    // return bp;
    EventResult::Number(bp)
}
