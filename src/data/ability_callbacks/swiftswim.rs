//! Swift Swim Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onModifySpe(spe, pokemon) {
///     if (['raindance', 'primordialsea'].includes(pokemon.effectiveWeather())) {
///         return this.chainModify(2);
///     }
/// }
pub fn on_modify_spe(battle: &mut Battle, _spe: i32, pokemon_pos: (usize, usize)) -> EventResult {
    // Get field weather
    let field_weather = battle.effective_weather();

    // Get pokemon and check effective weather
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    let eff_weather = pokemon.effective_weather(battle, field_weather.as_str());

    if eff_weather == "raindance" || eff_weather == "primordialsea" {
        battle.chain_modify(2.0); return EventResult::Continue;
    }

    EventResult::Continue
}

