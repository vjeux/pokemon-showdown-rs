//! Wildbolt Storm Move
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
pub fn on_modify_move(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    // if (target && ['raindance', 'primordialsea'].includes(target.effectiveWeather()))
    if let Some(target) = target_pos {
        let weather = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.effective_weather(battle, battle.field.weather.as_str())
        };

        if weather == "raindance" || weather == "primordialsea" {
            // move.accuracy = true;
            if let Some(ref mut active_move) = battle.active_move {
                active_move.accuracy = 0; // true in JS means always hit, represented as 0 in Rust
            }
        }
    }

    EventResult::Continue
}
