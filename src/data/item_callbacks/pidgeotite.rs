//! Pidgeotite Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTakeItem(item, source) {
///     if (item.megaEvolves === source.baseSpecies.baseSpecies) return false;
///     return true;
/// }
pub fn on_take_item(battle: &mut Battle, item_pos: Option<(usize, usize)>, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> EventResult {
    // if (item.megaEvolves === source.baseSpecies.baseSpecies) return false;
    // TODO: Need item.megaEvolves field and pokemon.baseSpecies.baseSpecies to check
    // If mega stone matches pokemon, should return EventResult::PreventDefault
    // This is documented in ITEMS_TODO.md as missing infrastructure
    EventResult::Continue
}
