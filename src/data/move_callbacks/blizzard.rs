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
pub fn on_modify_move(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // if (this.field.isWeather(['hail', 'snowscape'])) move.accuracy = true;
    if battle.field.is_weather_any(&["hail", "snowscape"]) {
        // move.accuracy = true;
        // Store accuracy override in current effect state
        if let Some(ref mut effect_state) = battle.current_effect_state {
            effect_state.data.insert(
                "accuracy".to_string(),
                serde_json::Value::Bool(true),
            );
        }
    }

    EventResult::Continue
}

