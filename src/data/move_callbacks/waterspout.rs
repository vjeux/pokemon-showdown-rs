//! Water Spout Move
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
    // Get current move's base power
    let base_power = match &battle.active_move {
        Some(active_move) => active_move.borrow().base_power,
        None => return EventResult::Continue,
    };

    // const bp = move.basePower * pokemon.hp / pokemon.maxhp;
    // JavaScript uses floating-point math, so we need to do the same
    let bp = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        // Use floating-point math like JavaScript
        let bp_float = (base_power as f64) * (pokemon.hp as f64) / (pokemon.maxhp as f64);
        bp_float
    };

    // this.debug(`BP: ${bp}`);
    battle.debug(&format!("BP: {}", bp));

    // return bp;
    // Return as Float since JavaScript returns a floating-point value
    EventResult::Float(bp)
}
