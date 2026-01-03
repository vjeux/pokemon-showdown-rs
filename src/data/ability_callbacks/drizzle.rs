//! Drizzle Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(source) {
///     if (source.species.id === 'kyogre' && source.item === 'blueorb') return;
///     this.field.setWeather('raindance');
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // Check if it's Kyogre with Blue Orb
    let is_kyogre_with_blue_orb = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.species_id.as_str() == "kyogre" && pokemon.item.as_str() == "blueorb"
    };

    if is_kyogre_with_blue_orb {
        return EventResult::Continue;
    }

    // Set weather to Rain Dance
    battle.field.set_weather(crate::ID::from("raindance"), None);
    EventResult::Continue
}

