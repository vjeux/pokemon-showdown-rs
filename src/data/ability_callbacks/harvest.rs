//! Harvest Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onResidual(pokemon) {
///     if (this.field.isWeather(['sunnyday', 'desolateland']) || this.randomChance(1, 2)) {
///         if (pokemon.hp && !pokemon.item && this.dex.items.get(pokemon.lastItem).isBerry) {
///             pokemon.setItem(pokemon.lastItem);
///             pokemon.lastItem = '';
///             this.add('-item', pokemon, pokemon.getItem(), '[from] ability: Harvest');
///         }
///     }
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

