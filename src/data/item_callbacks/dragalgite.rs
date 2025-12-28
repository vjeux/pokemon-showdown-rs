//! Dragalgite Item
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
    // return true;

    // Extract item ID and pokemon's base species base species
    let (item_id, pokemon_base_species_base_species) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        // Get the pokemon's base species
        let species = battle.dex.get_species(pokemon.base_species.as_str());
        let base_species_base_species = species
            .and_then(|s| s.base_species.clone())
            .unwrap_or_else(|| {
                // If base_species is None, use the species name itself
                species.map(|s| s.name.clone()).unwrap_or_default()
            });

        (pokemon.item.clone(), base_species_base_species)
    };

    // Get item data
    let item_data = battle.dex.get_item(item_id.as_str());
    let mega_evolves = item_data
        .and_then(|i| i.mega_evolves.clone())
        .unwrap_or_default();

    // if (item.megaEvolves === source.baseSpecies.baseSpecies) return false;
    if mega_evolves == pokemon_base_species_base_species {
        return EventResult::Boolean(false);
    }

    // return true;
    EventResult::Boolean(true)
}
