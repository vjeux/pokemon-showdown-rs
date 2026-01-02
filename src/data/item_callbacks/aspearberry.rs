//! Aspear Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onUpdate(pokemon) {
///     if (pokemon.status === 'frz') {
///         pokemon.eatItem();
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.status === 'frz') {
    //     pokemon.eatItem();
    // }

    let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    if pokemon_mut.status == "frz".into() {
        pokemon_mut.eat_item(false);
    }

    EventResult::Continue
}

/// onEat(pokemon) {
///     if (pokemon.status === 'frz') {
///         pokemon.cureStatus();
///     }
/// }
pub fn on_eat(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.status === 'frz') {
    //     pokemon.cureStatus();
    // }

    let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    if pokemon_mut.status == "frz".into() {
        pokemon_mut.cure_status(false);
    }

    EventResult::Continue
}
