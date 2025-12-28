//! Water Memory Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTakeItem(item, pokemon, source) {
///     if ((source && source.baseSpecies.num === 773) || pokemon.baseSpecies.num === 773) {
///         return false;
///     }
///     return true;
/// }
pub fn on_take_item(battle: &mut Battle, _item_pos: Option<(usize, usize)>, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> EventResult {
    // if ((source && source.baseSpecies.num === 773) || pokemon.baseSpecies.num === 773) {
    //     return false;
    // }
    // return true;

    // Check if pokemon is Silvally (num 773)
    let pokemon_is_silvally = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        battle.dex.get_species_by_id(&pokemon.base_species)
            .map(|species| species.num == 773)
            .unwrap_or(false)
    };

    // Check if source is Silvally (num 773)
    let source_is_silvally = if let Some(source) = source_pos {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        battle.dex.get_species_by_id(&source_pokemon.base_species)
            .map(|species| species.num == 773)
            .unwrap_or(false)
    } else {
        false
    };

    if source_is_silvally || pokemon_is_silvally {
        // return false;
        return EventResult::Boolean(false);
    }

    // return true;
    EventResult::Boolean(true)
}
