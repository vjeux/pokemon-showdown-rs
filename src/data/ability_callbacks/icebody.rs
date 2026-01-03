//! Ice Body Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onWeather(target, source, effect) {
///     if (effect.id === 'hail' || effect.id === 'snowscape') {
///         this.heal(target.baseMaxhp / 16);
///     }
/// }
pub fn on_weather(battle: &mut Battle, weather_id: &str, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>) -> EventResult {
    if weather_id == "hail" || weather_id == "snowscape" {
        // Get base max HP and calculate heal amount
        let heal_amount = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.base_maxhp / 16
        };

        // Heal the pokemon
        battle.heal(heal_amount, Some(pokemon_pos), None, None);
    }

    EventResult::Continue
}

/// onImmunity(type, pokemon) {
///     if (type === 'hail') return false;
/// }
pub fn on_immunity(_battle: &mut Battle, type_or_status: &str, _pokemon_pos: (usize, usize)) -> EventResult {
    if type_or_status == "hail" {
        return EventResult::Boolean(false);
    }
    EventResult::Continue
}

