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
    // NOTE: Must use battle.effective_weather() to account for Air Lock/Cloud Nine
    let field_weather = battle.effective_weather();
    let effective_weather = target_pokemon.effective_weather(battle, field_weather.as_str());

    if effective_weather == "raindance" || effective_weather == "primordialsea" {
        // move.accuracy = true;
        if let Some(ref mut active_move) = battle.active_move {
            active_move.borrow_mut().accuracy = crate::dex::Accuracy::AlwaysHits;
        }
    }

    EventResult::Continue
}
