//! Splash Plate Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, user, target, move) {
///     if (move.type === 'Water') {
///         return this.chainModify([4915, 4096]);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, _base_power: i32, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // if (move.type === 'Water')
    let is_water = {
        if let Some(ref active_move) = battle.active_move {
            active_move.move_type == "Water"
        } else {
            false
        }
    };

    if is_water {
        // return this.chainModify([4915, 4096]);
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
pub fn on_take_item(battle: &mut Battle, _item_pos: Option<(usize, usize)>, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> EventResult {
    // if ((source && source.baseSpecies.num === 493) || pokemon.baseSpecies.num === 493) {
    //     return false;
    // }

    // Check source if present
    if let Some(source) = source_pos {
        if let Some(source_pokemon) = battle.pokemon_at(source.0, source.1) {
            if let Some(species) = battle.dex.get_species_by_id(&source_pokemon.base_species) {
                if species.num == 493 {
                    return EventResult::Boolean(false);
                }
            }
        }
    }

    // Check pokemon
    if let Some(pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        if let Some(species) = battle.dex.get_species_by_id(&pokemon.base_species) {
            if species.num == 493 {
                return EventResult::Boolean(false);
            }
        }
    }

    // return true;
    EventResult::Boolean(true)
}
