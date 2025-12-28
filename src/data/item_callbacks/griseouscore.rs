//! Griseous Core Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, user, target, move) {
///     if (user.baseSpecies.num === 487 && (move.type === 'Ghost' || move.type === 'Dragon')) {
///         return this.chainModify([4915, 4096]);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, base_power: i32, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // if (user.baseSpecies.num === 487 && (move.type === 'Ghost' || move.type === 'Dragon'))

    // Check user.baseSpecies.num === 487
    let is_giratina = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        battle.dex.get_species(pokemon.base_species.as_str())
            .map(|s| s.num == 487)
            .unwrap_or(false)
    };

    if !is_giratina {
        return EventResult::Continue;
    }

    // move.type === 'Ghost' || move.type === 'Dragon'
    let is_ghost_or_dragon = battle.active_move.as_ref()
        .map(|m| m.move_type.as_str() == "Ghost" || m.move_type.as_str() == "Dragon")
        .unwrap_or(false);

    if is_ghost_or_dragon {
        // return this.chainModify([4915, 4096]);
        battle.chain_modify_fraction(4915, 4096);
    }

    EventResult::Continue
}

/// onTakeItem(item, pokemon, source) {
///     if (source?.baseSpecies.num === 487 || pokemon.baseSpecies.num === 487) {
///         return false;
///     }
///     return true;
/// }
pub fn on_take_item(battle: &mut Battle, item_pos: Option<(usize, usize)>, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> EventResult {
    // Check pokemon.baseSpecies.num === 487
    let pokemon_is_giratina = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        battle.dex.get_species(pokemon.base_species.as_str())
            .map(|s| s.num == 487)
            .unwrap_or(false)
    };

    // Check source?.baseSpecies.num === 487
    let source_is_giratina = if let Some(src_pos) = source_pos {
        let source = match battle.pokemon_at(src_pos.0, src_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        battle.dex.get_species(source.base_species.as_str())
            .map(|s| s.num == 487)
            .unwrap_or(false)
    } else {
        false
    };

    if source_is_giratina || pokemon_is_giratina {
        // return false;
        return EventResult::Boolean(false);
    }

    // return true;
    EventResult::Boolean(true)
}
