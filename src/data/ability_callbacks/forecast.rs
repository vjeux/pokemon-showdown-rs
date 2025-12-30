//! Forecast Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     this.singleEvent('WeatherChange', this.effect, this.effectState, pokemon);
/// }
pub fn on_start(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onWeatherChange(pokemon) {
///     if (pokemon.baseSpecies.baseSpecies !== 'Castform' || pokemon.transformed) return;
///     let forme = null;
///     switch (pokemon.effectiveWeather()) {
///     case 'sunnyday':
///     case 'desolateland':
///         if (pokemon.species.id !== 'castformsunny') forme = 'Castform-Sunny';
///         break;
///     case 'raindance':
///     case 'primordialsea':
///         if (pokemon.species.id !== 'castformrainy') forme = 'Castform-Rainy';
///         break;
///     case 'hail':
///     case 'snowscape':
///         if (pokemon.species.id !== 'castformsnowy') forme = 'Castform-Snowy';
///         break;
///     default:
///         if (pokemon.species.id !== 'castform') forme = 'Castform';
///         break;
///     }
///     if (pokemon.isActive && forme) {
///         pokemon.formeChange(forme, this.effect, false, '0', '[msg]');
///     }
/// }
pub fn on_weather_change(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

