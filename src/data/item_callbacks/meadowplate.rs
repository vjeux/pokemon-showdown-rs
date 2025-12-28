//! Meadow Plate Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, user, target, move) {
///     if (move.type === 'Grass') {
///         return this.chainModify([4915, 4096]);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, base_power: i32, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // if (move.type === 'Grass') {
    //     return this.chainModify([4915, 4096]);
    // }
    let move_type = match &battle.active_move {
        Some(active_move) => &active_move.move_type,
        None => return EventResult::Continue,
    };

    if move_type == "grass" {
        battle.chain_modify_fraction(4915, 4096);
    }

    EventResult::Continue
}

/// onTakeItem(item, pokemon, source) {
///     if ((source && source.baseSpecies.num === 493) || pokemon.baseSpecies.num === 493) {
///         return false;
///     }
///     return true;
/// }
pub fn on_take_item(battle: &mut Battle, item_pos: Option<(usize, usize)>, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> EventResult {
    // if ((source && source.baseSpecies.num === 493) || pokemon.baseSpecies.num === 493) {
    //     return false;
    // }
    // return true;

    // Check pokemon (the holder)
    let pokemon_species_num = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let species_id = &pokemon.base_species;
        let species_data = match battle.dex.get_species_by_id(species_id) {
            Some(s) => s,
            None => return EventResult::Continue,
        };

        species_data.num
    };

    if pokemon_species_num == 493 {
        return EventResult::Boolean(false);
    }

    // Check source if present
    if let Some(source) = source_pos {
        let source_species_num = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            let species_id = &source_pokemon.base_species;
            let species_data = match battle.dex.get_species_by_id(species_id) {
                Some(s) => s,
                None => return EventResult::Continue,
            };

            species_data.num
        };

        if source_species_num == 493 {
            return EventResult::Boolean(false);
        }
    }

    EventResult::Boolean(true)
}
