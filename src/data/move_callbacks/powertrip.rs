//! Power Trip Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target, move) {
///     const bp = move.basePower + 20 * pokemon.positiveBoosts();
///     this.debug(`BP: ${bp}`);
///     return bp;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    let move_id = match &battle.active_move {
        Some(id) => id,
        None => return EventResult::Continue,
    };

    let move_data = match battle.dex.get_move_by_id(move_id) {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // Count positive boosts
    let positive_boosts =
        std::cmp::max(0, pokemon.boosts.atk) +
        std::cmp::max(0, pokemon.boosts.def) +
        std::cmp::max(0, pokemon.boosts.spa) +
        std::cmp::max(0, pokemon.boosts.spd) +
        std::cmp::max(0, pokemon.boosts.spe) +
        std::cmp::max(0, pokemon.boosts.accuracy) +
        std::cmp::max(0, pokemon.boosts.evasion);

    let bp = move_data.base_power + (20 * positive_boosts);

    battle.debug(&format!("BP: {}", bp));
    EventResult::Number(bp)
}

