//! Dread Plate Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onBasePower(basePower, user, target, move) {
///     if (move && move.type === 'Dark') {
///         return this.chainModify([4915, 4096]);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, _base_power: i32, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // JavaScript checks move.type (the active move's type, not the dex type)
    let is_dark = battle.active_move.as_ref()
        .map(|m| m.move_type == "Dark")
        .unwrap_or(false);

    // if (move && move.type === 'Dark')
    if is_dark {
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
