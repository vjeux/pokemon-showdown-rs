//! Bleakwind Storm Move
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
    // if (target && ['raindance', 'primordialsea'].includes(target.effectiveWeather())) {
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let target_pokemon = match battle.pokemon_at(target.0, target.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Get the field weather and target's effective weather
    let field_weather = battle.field.weather.to_string();
    let effective_weather = target_pokemon.effective_weather(&field_weather);

    if effective_weather == "raindance" || effective_weather == "primordialsea" {
        // move.accuracy = true;
        // Store accuracy override in current effect state
        if let Some(ref mut effect_state) = battle.current_effect_state {
            effect_state
                .data
                .insert("accuracy".to_string(), serde_json::Value::Bool(true));
        }
    }

    EventResult::Continue
}
