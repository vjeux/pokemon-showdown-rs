//! PSN Cure Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onUpdate(pokemon) {
///     if (pokemon.status === 'psn' || pokemon.status === 'tox') {
///         pokemon.eatItem();
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.status === 'psn' || pokemon.status === 'tox')
    let has_poison = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let status = pokemon.status.as_str();
        status == "psn" || status == "tox"
    };

    // pokemon.eatItem();
    if has_poison {
        Pokemon::eat_item(battle, pokemon_pos, false, None, None);
    }

    EventResult::Continue
}

/// onEat(pokemon) {
///     if (pokemon.status === 'psn' || pokemon.status === 'tox') {
///         pokemon.cureStatus();
///     }
/// }
pub fn on_eat(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.status === 'psn' || pokemon.status === 'tox')
    let has_poison = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let status = pokemon.status.as_str();
        status == "psn" || status == "tox"
    };

    // pokemon.cureStatus();
    if has_poison {
        if let Some(_pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Pokemon::cure_status(battle, pokemon_pos, false);
        }
    }

    EventResult::Continue
}
