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
pub fn on_modify_type(battle: &mut Battle, move_id: &str, pokemon_pos: (usize, usize)) -> EventResult {
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Check if pokemon is ignoring its item
    if pokemon.ignoring_item(None) {
        return EventResult::Continue;
    }

    let item_data = pokemon.get_item();

    // Check if item has onPlate and is not a Z-Move
    if !item_data.id.is_empty() {
        if let Some(item) = battle.dex.items.get(item_data.id.as_str()) {
            if let Some(ref plate_type) = item.on_plate {
                // Check that it's not a Z-Move
                let is_z_move = item.z_move.is_some();
                if !is_z_move {
                    // TODO: Need battle.dex.getActiveMove(move_id) to set move.type
                    // For now, just debug the type that would be set
                    battle.debug(&format!("Judgment type: {}", plate_type));
                }
            }
        }
    }

    EventResult::Continue
}

