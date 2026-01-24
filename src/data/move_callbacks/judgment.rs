//! Judgment Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyType(move, pokemon) {
///     if (pokemon.ignoringItem()) return;
///     const item = pokemon.getItem();
///     if (item.id && item.onPlate && !item.zMove) {
///         move.type = item.onPlate;
///     }
/// }
pub fn on_modify_type(
    battle: &mut Battle,
    _active_move: Option<&crate::battle_actions::ActiveMove>,
    pokemon_pos: (usize, usize),
) -> EventResult {
    let pokemon = pokemon_pos;

    // if (pokemon.ignoringItem()) return;
    let ignoring_item = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.ignoring_item(battle, false)
    };

    if ignoring_item {
        return EventResult::Continue;
    }

    // const item = pokemon.getItem();
    // if (item.id && item.onPlate && !item.zMove) {
    //     move.type = item.onPlate;
    // }
    let on_plate_type = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let item_id = &pokemon_pokemon.item;

        let item = battle.dex.items().get_by_id(item_id);
        match item {
            Some(i) => {
                if i.on_plate.is_some() && i.z_move.is_none() {
                    i.on_plate.clone()
                } else {
                    None
                }
            }
            None => None,
        }
    };

    if let Some(plate_type) = on_plate_type {
        if let Some(ref mut active_move) = battle.active_move {
            active_move.borrow_mut().move_type = plate_type;
        }
    }

    EventResult::Continue
}
