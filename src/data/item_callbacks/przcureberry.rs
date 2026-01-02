//! PRZ Cure Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onUpdate(pokemon) {
///     if (pokemon.status === 'par') {
///         pokemon.eatItem();
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.status === 'par')
    let has_paralysis = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.status.as_str() == "par"
    };

    // pokemon.eatItem();
    if has_paralysis {
        if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            pokemon.eat_item(false, None, None);
        }
    }

    EventResult::Continue
}

/// onEat(pokemon) {
///     if (pokemon.status === 'par') {
///         pokemon.cureStatus();
///     }
/// }
pub fn on_eat(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.status === 'par')
    let has_paralysis = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.status.as_str() == "par"
    };

    // pokemon.cureStatus();
    if has_paralysis {
        if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            pokemon.cure_status(false);
        }
    }

    EventResult::Continue
}
