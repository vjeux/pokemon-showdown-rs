//! Thunder Move
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
pub fn on_modify_move(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // switch (target?.effectiveWeather())
    let weather = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.effective_weather(battle, battle.field.weather.as_str()).to_string()
    };

    // Modify move accuracy based on weather
    if let Some(ref mut active_move) = battle.active_move {
        match weather.as_ref() {
            // case 'raindance':
            // case 'primordialsea':
            //     move.accuracy = true;
            "raindance" | "primordialsea" => {
                active_move.accuracy = crate::dex::Accuracy::AlwaysHits;
            }
            // case 'sunnyday':
            // case 'desolateland':
            //     move.accuracy = 50;
            "sunnyday" | "desolateland" => {
                active_move.accuracy = crate::dex::Accuracy::Percent(50);
            }
            _ => {}
        }
    }

    EventResult::Continue
}
