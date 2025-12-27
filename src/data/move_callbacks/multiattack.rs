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
pub fn on_modify_type(battle: &mut Battle, move_id: &str, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;

    // if (pokemon.ignoringItem()) return;
    let ignoring_item = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.ignoring_item()
    };

    if ignoring_item {
        return EventResult::Continue;
    }

    // move.type = this.runEvent('Memory', pokemon, null, move, 'Normal');
    // Run the Memory event to get the new type
    // This event is handled by Memory items (like Fairy Memory, Dragon Memory, etc.)
    // The default return value is 'Normal' if no Memory item changes it
    let new_type = battle.run_event_with_default("Memory", pokemon, None, Some(&ID::from(move_id)), "Normal");

    // Set the move's type to the result
    if let Some(ref mut active_move) = battle.active_move {
        if let Some(type_str) = new_type.as_str() {
            active_move.move_type = Some(type_str.to_string());
        }
    }

    EventResult::Continue
}

