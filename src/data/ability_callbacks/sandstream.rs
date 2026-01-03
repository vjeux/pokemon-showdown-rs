//! Sand Stream Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(source) {
///     this.field.setWeather('sandstorm');
/// }
pub fn on_start(battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // Set weather to Sandstorm
    battle.field.set_weather(crate::ID::from("sandstorm"), None);
    EventResult::Continue
}

