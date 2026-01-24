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
    if battle.is_weather("hail") || battle.is_weather("snowscape") {
        // move.accuracy = true;
        if let Some(ref mut active_move) = battle.active_move {
            active_move.borrow_mut().accuracy = crate::dex::Accuracy::AlwaysHits;
        }
    }

    EventResult::Continue
}
