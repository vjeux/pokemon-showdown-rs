//! Red Orb Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSwitchIn(pokemon) {
///     if (pokemon.isActive && pokemon.baseSpecies.name === 'Groudon' && !pokemon.transformed) {
///         pokemon.formeChange('Groudon-Primal', this.effect, true);
///     }
/// }
pub fn on_switch_in(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.isActive && pokemon.baseSpecies.name === 'Groudon' && !pokemon.transformed) {
    //     pokemon.formeChange('Groudon-Primal', this.effect, true);
    // }
    // TODO: Need pokemon.isActive, pokemon.baseSpecies.name, pokemon.transformed,
    // and pokemon.formeChange() to transform Groudon to Primal form
    EventResult::Continue
}

/// onTakeItem(item, source) {
///     if (source.baseSpecies.baseSpecies === 'Groudon') return false;
///     return true;
/// }
pub fn on_take_item(battle: &mut Battle, item_pos: Option<(usize, usize)>, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> EventResult {
    // if (source.baseSpecies.baseSpecies === 'Groudon') return false;
    // TODO: Need source.baseSpecies.baseSpecies to check if Groudon
    // Should return EventResult::Boolean(false) to prevent taking if Groudon
    EventResult::Continue
}
