//! Growth Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyMove(move, pokemon) {
///     if (['sunnyday', 'desolateland'].includes(pokemon.effectiveWeather())) move.boosts = { atk: 2, spa: 2 };
/// }
pub fn on_modify_move(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;

    // if (['sunnyday', 'desolateland'].includes(pokemon.effectiveWeather())) move.boosts = { atk: 2, spa: 2 };
    let effective_weather = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.effective_weather()
    };

    if effective_weather == Some(ID::from("sunnyday")) || effective_weather == Some(ID::from("desolateland")) {
        // move.boosts = { atk: 2, spa: 2 };
        // Modify the current move's boosts
        if let Some(ref current_move_id) = battle.active_move {
            if let Some(current_move) = battle.dex.get_move_by_id_mut(current_move_id) {
                current_move.boosts.insert("atk".to_string(), 2);
                current_move.boosts.insert("spa".to_string(), 2);
            }
        }
    }

    EventResult::Continue
}

