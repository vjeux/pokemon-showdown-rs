//! Draco Plate Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, user, target, move) {
///     if (move && move.type === 'Dragon') {
///         return this.chainModify([4915, 4096]);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, _base_power: i32, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // Get the active move
    let move_type = match &battle.active_move {
        Some(active_move) => {
            // Get the move data to check its type
            match battle.dex.moves().get_by_id(&active_move.id) {
                Some(move_data) => &move_data.move_type,
                None => return EventResult::Continue,
            }
        }
        None => return EventResult::Continue,
    };

    // if (move && move.type === 'Dragon')
    if move_type == "Dragon" {
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
    // Get the pokemon
    let pokemon_num = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        // Get species data to check num
        match battle.dex.species().get_by_id(&pokemon.base_species) {
            Some(species) => species.num,
            None => return EventResult::Continue,
        }
    };

    // Get source num if present
    let source_num = if let Some((source_side, source_idx)) = source_pos {
        let source = match battle.pokemon_at(source_side, source_idx) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        match battle.dex.species().get_by_id(&source.base_species) {
            Some(species) => Some(species.num),
            None => None,
        }
    } else {
        None
    };

    // if ((source && source.baseSpecies.num === 493) || pokemon.baseSpecies.num === 493) return false;
    if source_num == Some(493) || pokemon_num == 493 {
        return EventResult::Boolean(false);
    }

    // return true;
    EventResult::Boolean(true)
}
