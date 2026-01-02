//! Ice Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onUpdate(pokemon) {
///     if (pokemon.status === 'brn') {
///         pokemon.eatItem();
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.status === 'brn')
    let is_burned = if let Some(pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        pokemon.status.as_str() == "brn"
    } else {
        return EventResult::Continue;
    };

    if is_burned {
        // pokemon.eatItem();
        if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            pokemon.eat_item(false, None, None);
        }
    }

    EventResult::Continue
}

/// onEat(pokemon) {
///     if (pokemon.status === 'brn') {
///         pokemon.cureStatus();
///     }
/// }
pub fn on_eat(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.status === 'brn')
    let is_burned = if let Some(pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        pokemon.status.as_str() == "brn"
    } else {
        return EventResult::Continue;
    };

    if is_burned {
        // pokemon.cureStatus();
        if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            pokemon.cure_status(false);
        }
    }

    EventResult::Continue
}
