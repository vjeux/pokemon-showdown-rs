//! Ghost Memory Item
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
pub fn on_take_item(battle: &mut Battle, item_pos: Option<(usize, usize)>, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> EventResult {
    // Get the pokemon
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Boolean(true),
    };

    // if ((source && source.baseSpecies.num === 773) || pokemon.baseSpecies.num === 773)
    let pokemon_num = pokemon.get_base_species_num(&battle.dex).unwrap_or(0);
    
    if let Some(source_pos_val) = source_pos {
        let source_pokemon = match battle.pokemon_at(source_pos_val.0, source_pos_val.1) {
            Some(p) => p,
            None => return EventResult::Boolean(true),
        };
        let source_num = source_pokemon.get_base_species_num(&battle.dex).unwrap_or(0);
        if source_num == 773 || pokemon_num == 773 {
            return EventResult::Boolean(false);
        }
    } else if pokemon_num == 773 {
        return EventResult::Boolean(false);
    }

    // return true;
    EventResult::Boolean(true)
}
