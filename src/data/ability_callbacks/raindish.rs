//! Rain Dish Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onWeather(target, source, effect) {
///     if (target.hasItem('utilityumbrella')) return;
///     if (effect.id === 'raindance' || effect.id === 'primordialsea') {
///         this.heal(target.baseMaxhp / 16);
///     }
/// }
pub fn on_weather(battle: &mut Battle, weather_id: &str, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>) -> EventResult {
    // Check if target has Utility Umbrella
    let has_umbrella = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.has_item(battle, &["utilityumbrella"])
    };

    if has_umbrella {
        return EventResult::Continue;
    }

    if weather_id == "raindance" || weather_id == "primordialsea" {
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

