//! Zap Plate Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, user, target, move) {
///     if (move.type === 'Electric') {
///         return this.chainModify([4915, 4096]);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, _base_power: i32, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // if (move.type === 'Electric') {
    //     return this.chainModify([4915, 4096]);
    // }
    let move_type = match &battle.active_move {
        Some(active_move) => active_move.borrow().move_type.clone(),
        None => return EventResult::Continue,
    };

    if move_type == "Electric" {
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
    // return true;

    // Check if pokemon is Arceus (num 493)
    let pokemon_is_arceus = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        battle.dex.species().get_by_id(&pokemon.base_species)
            .map(|species| species.num == 493)
            .unwrap_or(false)
    };

    // Check if source is Arceus (num 493)
    let source_is_arceus = if let Some(source) = source_pos {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        battle.dex.species().get_by_id(&source_pokemon.base_species)
            .map(|species| species.num == 493)
            .unwrap_or(false)
    } else {
        false
    };

    if source_is_arceus || pokemon_is_arceus {
        // return false;
        return EventResult::Boolean(false);
    }

    // return true;
    EventResult::Boolean(true)
}
