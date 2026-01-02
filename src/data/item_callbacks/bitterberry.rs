//! Bitter Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onUpdate(pokemon) {
///     if (pokemon.volatiles['confusion']) {
///         pokemon.eatItem();
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.volatiles['confusion']) {
    //     pokemon.eatItem();
    // }

    let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    if pokemon_mut.volatiles.contains_key(&"confusion".into()) {
        pokemon_mut.eat_item(false, None, None);
    }

    EventResult::Continue
}

/// onEat(pokemon) {
///     pokemon.removeVolatile('confusion');
/// }
pub fn on_eat(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // pokemon.removeVolatile('confusion');

    let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    pokemon_mut.remove_volatile(&"confusion".into());

    EventResult::Continue
}
