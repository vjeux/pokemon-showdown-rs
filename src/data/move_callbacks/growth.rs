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
        let field_weather = battle.field.weather.as_str();
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.effective_weather(field_weather)
    };

    if effective_weather == Some("sunnyday".to_string()) || effective_weather == Some("desolateland".to_string()) {
        // move.boosts = { atk: 2, spa: 2 };
        // Modify the current move's boosts
        if let Some(ref mut current_move) = battle.active_move {
            let mut boosts = crate::dex_data::BoostsTable::default();
            boosts.atk = 2;
            boosts.spa = 2;
            current_move.boosts = Some(boosts);
        }
    }

    EventResult::Continue
}

