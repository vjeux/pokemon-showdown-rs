//! Lum Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;
use crate::Pokemon;

/// onAfterSetStatus(status, pokemon) {
///     pokemon.eatItem();
/// }
pub fn on_after_set_status(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // pokemon.eatItem();

    let _pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    Pokemon::eat_item(battle, pokemon_pos, false, None, None);

    EventResult::Continue
}

/// onUpdate(pokemon) {
///     if (pokemon.status || pokemon.volatiles['confusion']) {
///         pokemon.eatItem();
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.status || pokemon.volatiles['confusion']) {
    //     pokemon.eatItem();
    // }

    let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    if !pokemon_mut.status.is_empty() || pokemon_mut.volatiles.contains_key(&"confusion".into()) {
        Pokemon::eat_item(battle, pokemon_pos, false, None, None);
    }

    EventResult::Continue
}

/// onEat(pokemon) {
///     pokemon.cureStatus();
///     pokemon.removeVolatile('confusion');
/// }
pub fn on_eat(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // pokemon.cureStatus();
    // pokemon.removeVolatile('confusion');

    Pokemon::cure_status(battle, pokemon_pos, false);
    Pokemon::remove_volatile(battle, pokemon_pos, &"confusion".into());

    EventResult::Continue
}
