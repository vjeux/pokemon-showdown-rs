//! Heatranite Item
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
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Boolean(true),
    };

    let item = match battle.dex.get_item("heatranite") {
        Some(item) => item,
        None => return EventResult::Boolean(true),
    };

    let source_pokemon = match battle.pokemon_at(source.0, source.1) {
        Some(p) => p,
        None => return EventResult::Boolean(true),
    };

    let source_base_species_base_species = match source_pokemon.get_base_species_base_species(&battle.dex) {
        Some(s) => s,
        None => return EventResult::Boolean(true),
    };

    if let Some(mega_evolves) = &item.mega_evolves {
        if mega_evolves == &source_base_species_base_species {
            return EventResult::Boolean(false);
        }
    }

    EventResult::Boolean(true)
    
}
