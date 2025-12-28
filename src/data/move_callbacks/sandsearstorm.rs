Fixed in sandsearstorm.rs: effective_weather = battle.effective_weather(target)
Total changes in sandsearstorm.rs: 1
//! Sandsear Storm Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyMove(move, pokemon, target) {
///     if (target && ['raindance', 'primordialsea'].includes(target.effectiveWeather())) {
///         move.accuracy = true;
///     }
/// }
pub fn on_modify_move(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    // if (target && ['raindance', 'primordialsea'].includes(target.effectiveWeather())) {
    //     move.accuracy = true;
    // }
    if let Some(target) = target_pos {
        let effective_weather = {
            let field_weather = battle.field.weather.as_str();
            let pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let weather_str = pokemon.effective_weather(field_weather);
            if weather_str.is_empty() {
                None
            } else {
                Some(ID::from(weather_str.as_str()))
            }
        };

        if effective_weather == Some(ID::from("raindance")) || effective_weather == Some(ID::from("primordialsea")) {
            let active_move = match &mut battle.active_move {
                Some(m) => m,
                None => return EventResult::Continue,
            };
            active_move.accuracy = 0; // true in TS means always hit, represented as 0 in the accuracy field
        }
    }

    EventResult::Continue
}


