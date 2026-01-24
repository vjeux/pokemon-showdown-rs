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
pub fn on_modify_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    let pokemon = pokemon_pos;

    // if (['sunnyday', 'desolateland'].includes(pokemon.effectiveWeather())) move.boosts = { atk: 2, spa: 2 };
    // NOTE: Must use battle.effective_weather() to account for Air Lock/Cloud Nine
    let field_weather = battle.effective_weather();
    let effective_weather = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.effective_weather(battle, field_weather.as_str())
    };

    if effective_weather == "sunnyday" || effective_weather == "desolateland" {
        // move.boosts = { atk: 2, spa: 2 };
        // Modify the current move's boosts
        if let Some(ref current_move) = battle.active_move {
            let boosts = crate::dex_data::BoostsTable {
                atk: 2,
                spa: 2,
                ..Default::default()
            };
            current_move.borrow_mut().boosts = Some(boosts);
        }
    }

    EventResult::Continue
}
