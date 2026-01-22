//! Snow Warning Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onStart(source) {
///     this.field.setWeather('snowscape');
/// }
pub fn on_start(battle: &mut Battle, _pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect: Option<&Effect>) -> EventResult {
    // Set weather to Snowscape
    battle.set_weather(crate::ID::from("snowscape"), None, None);
    EventResult::Continue
}

