//! Custap Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onFractionalPriority(priority, pokemon) {
///     if (
///         priority <= 0 &&
///         (pokemon.hp <= pokemon.maxhp / 4 || (pokemon.hp <= pokemon.maxhp / 2 &&
///             pokemon.hasAbility('gluttony') && pokemon.abilityState.gluttony))
///     ) {
///         if (pokemon.eatItem()) {
///             this.add('-activate', pokemon, 'item: Custap Berry', '[consumed]');
///             return 0.1;
///         }
///     }
/// }
pub fn on_fractional_priority(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onEat() {
///     num: 210,
///     gen: 4,
/// }
pub fn on_eat(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}
