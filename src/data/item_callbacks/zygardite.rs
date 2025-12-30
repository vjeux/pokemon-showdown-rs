//! Zygardite Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTakeItem(item, source) {
///     if (source.baseSpecies.baseSpecies === 'Zygarde') return false;
///     return true;
/// }
pub fn on_take_item(battle: &mut Battle, _item_pos: Option<(usize, usize)>, _pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> EventResult {
    // if (source.baseSpecies.baseSpecies === 'Zygarde') return false;
    // return true;

    // Extract source's base species base species
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Boolean(true),
    };

    let source_base_species_base_species = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Boolean(true),
        };

        // Get the source's base species
        let species = battle.dex.get_species(source_pokemon.base_species.as_str());
        species
            .and_then(|s| s.base_species.clone())
            .unwrap_or_else(|| {
                // If base_species is None, use the species name itself
                species.map(|s| s.name.clone()).unwrap_or_default()
            })
    };

    // if (source.baseSpecies.baseSpecies === 'Zygarde') return false;
    if source_base_species_base_species == "Zygarde" {
        return EventResult::Boolean(false);
    }

    // return true;
    EventResult::Boolean(true)
}
