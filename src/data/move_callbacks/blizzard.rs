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
        // In JavaScript, this sets accuracy to boolean true which means always hit
        // In Rust, we need to modify battle.active_move.accuracy directly
        if let Some(ref mut active_move) = battle.active_move {
            active_move.accuracy = crate::dex::Accuracy::AlwaysHits;
        }
    }

    EventResult::Continue
}
