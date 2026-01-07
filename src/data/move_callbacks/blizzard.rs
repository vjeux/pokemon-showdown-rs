//! Blizzard Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyMove(move) {
///     if (this.field.isWeather(['hail', 'snowscape'])) move.accuracy = true;
/// }
pub fn on_modify_move(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // if (this.field.isWeather(['hail', 'snowscape'])) move.accuracy = true;
    // JavaScript isWeather accepts array - check each weather
    if battle.is_weather("hail") || battle.is_weather("snowscape") {
        // move.accuracy = true;
        // Store accuracy override in current effect state
        battle.with_effect_state(|state| {
            state.data.insert("accuracy".to_string(), serde_json::Value::Bool(true));
        });
    }

    EventResult::Continue
}
