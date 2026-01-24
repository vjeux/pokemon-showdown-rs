//! Multi-Attack Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyType(move, pokemon) {
///     if (pokemon.ignoringItem()) return;
///     move.type = this.runEvent('Memory', pokemon, null, move, 'Normal');
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

    // move.type = this.runEvent('Memory', pokemon, null, move, 'Normal');
    // Run the Memory event to get the new type
    // This event is handled by Memory items (like Fairy Memory, Dragon Memory, etc.)
    // The Memory event checks the item's onMemory field for the type
    let new_type = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        // Check if Pokemon holds a Memory item with onMemory field
        let item_id = &pokemon_pokemon.item;
        if !item_id.is_empty() {
            if let Some(item_data) = battle.dex.items().get_by_id(item_id) {
                // Memory items have onMemory field that specifies the type
                if let Some(ref memory_type) = item_data.on_memory {
                    memory_type.as_str()
                } else {
                    "Normal"
                }
            } else {
                "Normal"
            }
        } else {
            "Normal"
        }
    };

    // Set the move's type to the result
    if let Some(ref mut active_move) = battle.active_move {
        active_move.borrow_mut().move_type = new_type.to_owned();
    }

    EventResult::Continue
}
