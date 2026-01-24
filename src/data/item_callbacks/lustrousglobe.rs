//! Lustrous Globe Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, user, target, move) {
///     if (user.baseSpecies.num === 484 && (move.type === 'Water' || move.type === 'Dragon')) {
///         return this.chainModify([4915, 4096]);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, _base_power: i32, pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // if (user.baseSpecies.num === 484 && (move.type === 'Water' || move.type === 'Dragon'))
    let (species_num, move_type) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let species_num = battle.dex.species().get(pokemon.base_species.as_str())
            .map(|s| s.num)
            .unwrap_or(0);
        let move_type = battle.active_move.as_ref()
            .map(|m| m.borrow().move_type.clone())
            .unwrap_or_default();
        (species_num, move_type)
    };

    if species_num == 484 && (move_type == "Water" || move_type == "Dragon") {
        // return this.chainModify([4915, 4096]);
        battle.chain_modify_fraction(4915, 4096);
    }

    EventResult::Continue
}

/// onTakeItem(item, pokemon, source) {
///     if (source?.baseSpecies.num === 484 || pokemon.baseSpecies.num === 484) {
///         return false;
///     }
///     return true;
/// }
pub fn on_take_item(battle: &mut Battle, _item_pos: Option<(usize, usize)>, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> EventResult {
    // if (source?.baseSpecies.num === 484 || pokemon.baseSpecies.num === 484)

    // Check source if present
    if let Some(source) = source_pos {
        if let Some(source_pokemon) = battle.pokemon_at(source.0, source.1) {
            let source_species = battle.dex.species().get(source_pokemon.base_species.as_str());
            if let Some(species_data) = source_species {
                if species_data.num == 484 {
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
            if species_data.num == 484 {
                // return false;
                return EventResult::Boolean(false);
            }
        }
    }

    // return true;
    EventResult::Boolean(true)
}
