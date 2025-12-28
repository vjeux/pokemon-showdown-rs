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
    let pokemon = pokemon_pos;

    // const bp = move.basePower + 20 * pokemon.positiveBoosts();
    let bp = {
        let base_power = {
            let active_move = match &battle.active_move {
                Some(m) => m,
                None => return EventResult::Continue,
            };
            active_move.base_power
        };

        let positive_boosts = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            // Count all positive boosts (atk, def, spa, spd, spe, accuracy, evasion)
            let mut count = 0i32;
            if pokemon_pokemon.boosts.atk > 0 { count += pokemon_pokemon.boosts.atk; }
            if pokemon_pokemon.boosts.def > 0 { count += pokemon_pokemon.boosts.def; }
            if pokemon_pokemon.boosts.spa > 0 { count += pokemon_pokemon.boosts.spa; }
            if pokemon_pokemon.boosts.spd > 0 { count += pokemon_pokemon.boosts.spd; }
            if pokemon_pokemon.boosts.spe > 0 { count += pokemon_pokemon.boosts.spe; }
            if pokemon_pokemon.boosts.accuracy > 0 { count += pokemon_pokemon.boosts.accuracy; }
            if pokemon_pokemon.boosts.evasion > 0 { count += pokemon_pokemon.boosts.evasion; }
            count
        };

        base_power + 20 * positive_boosts
    };

    // this.debug(`BP: ${bp}`);
    battle.debug(&format!("BP: {}", bp));

    // return bp;
    EventResult::Number(bp)
}

