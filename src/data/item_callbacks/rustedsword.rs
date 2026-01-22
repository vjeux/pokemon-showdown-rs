//! Rusted Sword Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTakeItem(item, pokemon, source) {
///     if ((source && source.baseSpecies.num === 888) || pokemon.baseSpecies.num === 888) {
///         return false;
///     }
///     return true;
/// }
pub fn on_take_item(battle: &mut Battle, _item_pos: Option<(usize, usize)>, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> EventResult {
    // if ((source && source.baseSpecies.num === 888) || pokemon.baseSpecies.num === 888) {
    //     return false;
    // }

    // Check source if present
    if let Some(source) = source_pos {
        if let Some(source_pokemon) = battle.pokemon_at(source.0, source.1) {
            if let Some(species) = battle.dex.species().get_by_id(&source_pokemon.base_species) {
                if species.num == 888 {
                    return EventResult::Boolean(false);
                }
            }
        }
    }

    // Check pokemon
    if let Some(pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        if let Some(species) = battle.dex.species().get_by_id(&pokemon.base_species) {
            if species.num == 888 {
                return EventResult::Boolean(false);
            }
        }
    }

    // return true;
    EventResult::Boolean(true)
}
