//! Douse Drive Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onTakeItem(item, pokemon, source) {
///     if ((source && source.baseSpecies.num === 649) || pokemon.baseSpecies.num === 649) {
///         return false;
///     }
///     return true;
/// }
pub fn on_take_item(battle: &mut Battle, _item_pos: Option<(usize, usize)>, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> EventResult {
    // if ((source && source.baseSpecies.num === 649) || pokemon.baseSpecies.num === 649)

    // Check source if present
    if let Some(source) = source_pos {
        if let Some(source_pokemon) = battle.pokemon_at(source.0, source.1) {
            let source_species = battle.dex.species().get(source_pokemon.base_species.as_str());
            if let Some(species_data) = source_species {
                if species_data.num == 649 {
                    // return false;
                    return EventResult::Boolean(false);
                }
            }
        }
    }

    // Check pokemon
    if let Some(pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        let pokemon_species = battle.dex.species().get(pokemon.base_species.as_str());
        if let Some(species_data) = pokemon_species {
            if species_data.num == 649 {
                // return false;
                return EventResult::Boolean(false);
            }
        }
    }

    // return true;
    EventResult::Boolean(true)
}
