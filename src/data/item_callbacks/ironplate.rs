//! Iron Plate Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onBasePower(basePower, user, target, move) {
///     if (move.type === 'Steel') {
///         return this.chainModify([4915, 4096]);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, _base_power: i32, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // if (move.type === 'Steel')
    let is_steel = battle.active_move.as_ref()
        .map(|m| m.move_type.as_str() == "Steel")
        .unwrap_or(false);

    if is_steel {
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
    // Check pokemon.baseSpecies.num === 493
    let pokemon_is_arceus = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        battle.dex.species().get(pokemon.base_species.as_str())
            .map(|s| s.num == 493)
            .unwrap_or(false)
    };

    // Check source?.baseSpecies.num === 493
    let source_is_arceus = if let Some(src_pos) = source_pos {
        let source = match battle.pokemon_at(src_pos.0, src_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        battle.dex.species().get(source.base_species.as_str())
            .map(|s| s.num == 493)
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
