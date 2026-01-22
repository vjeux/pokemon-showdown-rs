//! Drought Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onStart(source) {
///     if (source.species.id === 'groudon' && source.item === 'redorb') return;
///     this.field.setWeather('sunnyday');
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect: Option<&Effect>) -> EventResult {
    // Check if it's Groudon with Red Orb
    let is_groudon_with_red_orb = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.species_id.as_str() == "groudon" && pokemon.item.as_str() == "redorb"
    };

    if is_groudon_with_red_orb {
        return EventResult::Continue;
    }

    // Set weather to Sunny Day
    battle.set_weather(crate::ID::from("sunnyday"), None, None);
    EventResult::Continue
}

