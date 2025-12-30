//! Raichunite X Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTakeItem(item, source) {
///     if (item.megaEvolves === source.baseSpecies.name ||
///         item.megaStone === source.baseSpecies.name) return false;
///     return true;
/// }
pub fn on_take_item(battle: &mut Battle, item_pos: Option<(usize, usize)>, _pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> EventResult {
    // if (item.megaEvolves === source.baseSpecies.name ||
    //     item.megaStone === source.baseSpecies.name) return false;
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

    // Get item data
    let (mega_evolves, mega_stone) = {
        let item_data = match battle.dex.get_item_by_id(&item_id) {
            Some(item) => item,
            None => return EventResult::Boolean(true),
        };
        (item_data.mega_evolves.clone(), item_data.mega_stone.clone())
    };

    // Get source's base species name
    let source_base_species_name = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Boolean(true),
        };

        let species = battle.dex.species().get(source_pokemon.base_species.as_str());
        species.map(|s| s.name.clone()).unwrap_or_default()
    };

    // if (item.megaEvolves === source.baseSpecies.name || item.megaStone === source.baseSpecies.name) return false;
    if let Some(evolves) = mega_evolves {
        if evolves == source_base_species_name {
            return EventResult::Boolean(false);
        }
    }
    if let Some(stone) = mega_stone {
        if stone == source_base_species_name {
            return EventResult::Boolean(false);
        }
    }

    // return true;
    EventResult::Boolean(true)
}
