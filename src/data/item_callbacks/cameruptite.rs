//! Cameruptite Item
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
pub fn on_take_item(battle: &mut Battle, _item_pos: Option<(usize, usize)>, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>) -> EventResult {
    // Get the item ID from current_effect
    let item_id = match &battle.current_effect {
        Some(id) => id,
        None => return EventResult::Continue,
    };

    // Get the item data
    let mega_evolves = match battle.dex.get_item_by_id(item_id) {
        Some(item) => match &item.mega_evolves {
            Some(me) => me.clone(),
            None => return EventResult::Continue,
        },
        None => return EventResult::Continue,
    };

    // Get the pokemon (source)
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Get the base species data
    let base_species_base = match battle.dex.get_species_by_id(&pokemon.base_species) {
        Some(species) => match &species.base_species {
            Some(bs) => bs.clone(),
            None => pokemon.base_species.to_string(),
        },
        None => return EventResult::Continue,
    };

    // if (item.megaEvolves === source.baseSpecies.baseSpecies) return false;
    if mega_evolves == base_species_base {
        return EventResult::Bool(false);
    }

    // return true;
    EventResult::Bool(true)
}
