//! Hurricane Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyMove(move, pokemon, target) {
///     switch (target?.effectiveWeather()) {
///     case 'raindance':
///     case 'primordialsea':
///         move.accuracy = true;
///         break;
///     case 'sunnyday':
///     case 'desolateland':
///         move.accuracy = 50;
///         break;
///     }
/// }
pub fn on_modify_move(battle: &mut Battle, _pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    

    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // switch (target?.effectiveWeather()) {
    let effective_weather = {
        let field_weather = battle.field.weather.as_str();
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.effective_weather(field_weather)
    };

    // case 'raindance':
    // case 'primordialsea':
    //     move.accuracy = true;
    //     break;
    // case 'sunnyday':
    // case 'desolateland':
    //     move.accuracy = 50;
    //     break;
    let weather_id = effective_weather;
    if weather_id == "raindance" || weather_id == "primordialsea" {
        if let Some(ref mut active_move) = battle.active_move {
            active_move.accuracy = 0;
        }
    } else if weather_id == "sunnyday" || weather_id == "desolateland" {
        if let Some(ref mut active_move) = battle.active_move {
            active_move.accuracy = 50;
        }
    }

    EventResult::Continue
}

