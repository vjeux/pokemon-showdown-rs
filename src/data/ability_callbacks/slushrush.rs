//! Slush Rush Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifySpe(spe, pokemon) {
///     if (this.field.isWeather(['hail', 'snowscape'])) {
///         return this.chainModify(2);
///     }
/// }
pub fn on_modify_spe(battle: &mut Battle, _spe: i32, _pokemon_pos: (usize, usize)) -> EventResult {
    if battle.field.is_weather_any(&["hail", "snowscape"]) {
        return EventResult::Number(battle.chain_modify(2.0));
    }
    EventResult::Continue
}

