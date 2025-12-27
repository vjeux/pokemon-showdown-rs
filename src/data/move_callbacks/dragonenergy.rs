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
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let pokemon = pokemon_pos;

    // Get current move's base power
    let base_power = match &battle.current_move {
        Some(move_id) => {
            match battle.dex.get_move_by_id(move_id) {
                Some(move_data) => move_data.base_power,
                None => return EventResult::Continue,
            }
        }
        None => return EventResult::Continue,
    };

    // const bp = move.basePower * pokemon.hp / pokemon.maxhp;
    let bp = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        base_power * pokemon_pokemon.hp / pokemon_pokemon.max_hp
    };

    // this.debug(`BP: ${bp}`);
    battle.debug(&format!("BP: {}", bp));

    // return bp;
    EventResult::Int(bp)
}

