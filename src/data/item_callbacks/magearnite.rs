//! Magearnite Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTakeItem(item, source) {
///     if (item.megaEvolves!.includes(source.baseSpecies.baseSpecies)) return false;
///     return true;
/// }
pub fn on_take_item(battle: &mut Battle, item_pos: Option<(usize, usize)>, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> EventResult {
    // if (item.megaEvolves!.includes(source.baseSpecies.baseSpecies)) return false;
    // return true;

    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Boolean(true),
    };

    // Get the item being taken
    let item_id = match item_pos {
        Some(pos) => {
            let pokemon = match battle.pokemon_at(pos.0, pos.1) {
                Some(p) => p,
                None => return EventResult::Boolean(true),
            };
            pokemon.item.clone()
        }
        None => return EventResult::Boolean(true),
    };

    // Get item's megaEvolves
    let mega_evolves = {
        let item_data = match battle.dex.get_item_by_id(&item_id) {
            Some(item) => item,
            None => return EventResult::Boolean(true),
        };
        item_data.mega_evolves.clone().unwrap_or_default()
    };

    // Get source's base species base species
    let source_base_species_base_species = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Boolean(true),
        };

        let species = battle.dex.get_species(source_pokemon.base_species.as_str());
        species
            .and_then(|s| s.base_species.clone())
            .unwrap_or_else(|| {
                species.map(|s| s.name.clone()).unwrap_or_default()
            })
    };

    // if (item.megaEvolves!.includes(source.baseSpecies.baseSpecies)) return false;
    if mega_evolves.contains(&source_base_species_base_species) {
        return EventResult::Boolean(false);
    }

    // return true;
    EventResult::Boolean(true)
}
