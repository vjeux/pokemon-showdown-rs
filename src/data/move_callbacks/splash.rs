//! Splash Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Arg, Battle};
use crate::event::EventResult;
use crate::dex_data::ID;

/// onTry(source, target, move) {
///     // Additional Gravity check for Z-move variant
///     if (this.field.getPseudoWeather('Gravity')) {
///         this.add('cant', source, 'move: Gravity', move);
///         return null;
///     }
/// }
pub fn on_try(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // Additional Gravity check for Z-move variant
    if battle.field.get_pseudo_weather(&ID::new("gravity")).is_some() {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let move_name = battle.active_move.as_ref()
            .and_then(|id| battle.dex.moves.get(id.as_str()))
            .map(|m| m.name.as_str())
            .unwrap_or("Splash");

        battle.add("cant", &[
            Arg::Pokemon(source),
            Arg::Str("move: Gravity"),
            Arg::String(move_name.to_string()),
        ]);
        return EventResult::Null;
    }

    EventResult::Continue
}

/// onTryHit(target, source) {
///     this.add('-nothing');
/// }
pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    battle.add("-nothing", &[]);
    EventResult::Continue
}

